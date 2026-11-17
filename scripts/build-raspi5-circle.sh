#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
repo="$PWD"
root="$(realpath "${1:?Usage: $0 ROOT.vfs [embedded|sd]}")"
test -s "$root"
mode="${2:-embedded}"
case "$mode" in
  embedded) board=raspi5-circle; features=(); root_define=; fatfs= ;;
  sd) board=raspi5-circle-sd; features=(--features sd-root); root_define=-DREI_ROOT_SD; fatfs=addon/fatfs ;;
  *) echo "Root source must be embedded or sd" >&2; exit 2 ;;
esac
prefix="${PREFIX64:-aarch64-none-elf-}"
jobs="${REI_JOBS:-2}"
[[ "$jobs" =~ ^[1-4]$ ]] || { echo "REI_JOBS must be 1 to 4" >&2; exit 2; }
for tool in "${prefix}gcc" "${prefix}g++" "${prefix}ld" "${prefix}nm" cargo rustc make; do
  command -v "$tool" >/dev/null || { echo "Missing tool: $tool" >&2; exit 1; }
done

git submodule sync -- thistle vendor/circle vendor/rei-platform
git submodule update --init --jobs "$jobs" -- thistle vendor/circle vendor/rei-platform
pin() { git ls-tree HEAD "$1" | awk '$1 == "160000" {print $3}'; }
for dep in thistle vendor/circle vendor/rei-platform; do
  test "$(git -C "$dep" rev-parse HEAD)" = "$(pin "$dep")"
  test -z "$(git -C "$dep" status --porcelain --untracked-files=no --ignore-submodules=all)"
done
for dep in thistle bake vendor/circle; do
  test "$(pin "$dep")" = "$(git -C vendor/rei-platform ls-tree HEAD "$dep" | awk '$1 == "160000" {print $3}')" || {
    echo "C and Rust platform pins diverge: $dep" >&2; exit 1;
  }
done
for dep in thistle bake crust; do
  grep -Fq "| ${dep^} | $(pin "$dep") |" docs/provenance.md
done

pinned="$(pin vendor/circle)"
key="$(printf '%s\n' "$pinned" "$prefix" "$("${prefix}gcc" -dumpfullversion)" 'pi5-16m-v2' | sha256sum | cut -c1-16)"
circle="$repo/build/circle/$key"
if [[ ! -f "$circle/Rules.mk" ]]; then
  mkdir -p "$circle"
  git -C vendor/circle archive "$pinned" | tar -xf - -C "$circle"
fi
cat > "$circle/Config.mk" <<EOF
AARCH = 64
RASPPI = 5
PREFIX64 = $prefix
STDLIB_SUPPORT = 1
DEFINE += -DKERNEL_MAX_SIZE=16777216 -DDEFAULT_KEYMAP=\"UK\"
EOF

if [[ "$mode" == sd ]]; then
  sed -i 's/^#define FF_FS_READONLY[[:space:]]*0$/#define FF_FS_READONLY 1/' "$circle/addon/fatfs/ffconf.h"
  grep -Eq '^#define FF_FS_READONLY[[:space:]]+1$' "$circle/addon/fatfs/ffconf.h"
fi
for library in lib lib/usb lib/input lib/fs addon/SDCard $fatfs; do
  nice -n 10 make -C "$circle/$library" -j "$jobs"
done

out="$repo/build/native/$board"
mkdir -p "$out"
if [[ "$root" != "$out/root.vfs" ]]; then cp "$root" "$out/root.vfs"; fi
adapter="$repo/vendor/rei-platform/platform"
cp "$adapter/raspi5/circle/console.cpp" "$adapter/raspi5/circle/storage.h" "$out/"
if [[ "$mode" == embedded ]]; then
  printf '.section .rodata.root,"a"\n.global rei_root_start,rei_root_end\n.balign 16\nrei_root_start:\n.incbin "root.vfs"\nrei_root_end:\n' > "$out/root.S"
else
  printf '.section .rodata.root,"a"\n.global rei_root_start,rei_root_end\n.balign 16\nrei_root_start:\nrei_root_end:\n' > "$out/root.S"
fi

# The only guest kernel library is the generated Rust crate.
# Do not compile or link C kernel source from the platform dependency.
RUSTFLAGS="-C target-cpu=cortex-a76 -C relocation-model=static -D warnings" \
  nice -n 10 cargo build --release --manifest-path platform/raspi5/Cargo.toml \
    --target aarch64-unknown-none --target-dir "$repo/build/rust-pi/$mode" \
    -j "$jobs" "${features[@]}"
rustlib="$repo/build/rust-pi/$mode/aarch64-unknown-none/release/librusty_rei_raspi5.a"
test -s "$rustlib"
fatfs_lib=
if [[ "$mode" == sd ]]; then fatfs_lib="$circle/addon/fatfs/libfatfs.a"; fi
rm -f "$out/console.o" "$out/root.o"
target="rusty-rei-$board"
nice -n 10 make -C "$out" -f "$repo/platform/raspi5/Makefile" \
  CIRCLEHOME="$circle" EXTRAINCLUDE="-I$adapter/arm64 $root_define" \
  REI_FATFS_LIB="$fatfs_lib" REI_KERNEL_LIB="$rustlib" OBJS="console.o root.o" \
  TARGET="$target" -j "$jobs" "$target.img"

end="$("${prefix}nm" "$out/$target.elf" | awk '$3 == "_end" {print $1}')"
[[ "$end" =~ ^[0-9a-fA-F]+$ ]] && (( 16#$end <= 0x80000 + 16777216 )) || {
  echo "Circle kernel exceeds reserved image memory" >&2; exit 1;
}
"${prefix}nm" "$out/$target.elf" | grep -E ' [Tt] rei_native_start$' >/dev/null
{
  printf 'rusty-rei %s\n' "$(git rev-parse HEAD)"
  for dep in thistle bake crust vendor/circle vendor/rei-platform; do printf '%s %s\n' "$dep" "$(pin "$dep")"; done
  printf 'root-source %s\n' "$mode"
  rustc --version
} > "$out/revisions.txt"
(cd "$out"; sha256sum "$target.img" "$target.elf" root.vfs revisions.txt > SHA256SUMS)
echo "Rusty Rei Pi 5 image: $out/$target.img"
