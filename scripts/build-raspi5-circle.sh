#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
repo="$PWD"
root="$(realpath "${1:?Usage: $0 ROOT.vfs [embedded|sd]}")"
test -s "$root"
mode="${2:-embedded}"
case "$mode" in
  embedded) board=raspi5-circle; features=(); root_define= ;;
  sd) board=raspi5-circle-sd; features=(--features sd-root); root_define=-DREI_ROOT_SD ;;
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
platform_pin() { git -C vendor/rei-platform ls-tree HEAD "$1" | awk '$1 == "160000" {print $3}'; }
test "$(pin vendor/circle)" = "$(platform_pin vendor/circle)" || {
  echo "Circle source revisions differ" >&2; exit 1;
}
# The hardware bundle does not depend on either generated kernel.
# Record differing kernel inputs; do not claim kernel parity.
for dep in thistle bake; do
  if [[ "$(pin "$dep")" != "$(platform_pin "$dep")" ]]; then
    echo "Warning: C and Rust kernel pins differ: $dep" >&2
  fi
done
for dep in thistle bake crust; do
  grep -Fq "| ${dep^} | $(pin "$dep") |" docs/provenance.md
done

# Use the exact verification and configuration functions from C Rei.
# Never invoke circle_compile or circle_export here.
source "$repo/vendor/rei-platform/scripts/raspi5-driver-bundle.sh"
circle_prepare
bundle="$repo/vendor/rei-platform/prebuilt/raspi5"
circle_verify "$bundle"
echo "RUSTY-REI: using verified precompiled Circle drivers ($bundle)"

out="$repo/build/native/$board"
mkdir -p "$out"
if [[ "$root" != "$out/root.vfs" ]]; then cp "$root" "$out/root.vfs"; fi
adapter="$repo/vendor/rei-platform/platform"
cp "$adapter"/raspi5/circle/{console.cpp,storage.h,network.cpp,network.h} "$out/"
cp "$bundle/circle-drivers.o" "$bundle/driver-archives.txt" "$out/"
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
rm -f "$out/console.o" "$out/network.o" "$out/root.o"
target="rusty-rei-$board"
nice -n 10 make -C "$out" -f "$repo/platform/raspi5/Makefile" \
  CIRCLEHOME="$circle" EXTRAINCLUDE="-I$adapter/arm64 $root_define" \
  REI_DRIVER_BUNDLE="$bundle" REI_KERNEL_LIB="$rustlib" OBJS="console.o network.o root.o circle-drivers.o" \
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
  printf 'circle-bundle %s\n' "$(sha256sum "$bundle/SHA256SUMS" | cut -d' ' -f1)"
  for dep in thistle bake; do printf 'rei-platform-%s %s\n' "$dep" "$(platform_pin "$dep")"; done
  rustc --version
} > "$out/revisions.txt"
(cd "$out"; sha256sum "$target.img" "$target.elf" root.vfs revisions.txt circle-drivers.o driver-archives.txt > SHA256SUMS)
echo "Rusty Rei Pi 5 image: $out/$target.img"
