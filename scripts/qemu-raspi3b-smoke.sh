#!/usr/bin/env bash
set -euo pipefail

# A bounded bare-metal AArch64 Raspberry Pi 3B test of a generated Thistle
# function. This is deliberately not described as a full kernel boot.
cd "$(dirname "${BASH_SOURCE[0]}")/.."

case "${1:-}" in
  "") build_only=false ;;
  --build-only) build_only=true ;;
  *) echo "Usage: $0 [--build-only]" >&2; exit 2 ;;
esac
commands=(clang ld.lld llvm-objcopy)
if ! $build_only; then
  commands+=(qemu-system-aarch64 timeout grep)
fi
for command in "${commands[@]}"; do
  if ! command -v "$command" >/dev/null; then
    echo "Missing required command: $command" >&2
    exit 2
  fi
done

qemu_seconds="${REI_QEMU_TIMEOUT:-8}"
if [[ ! "$qemu_seconds" =~ ^[1-9][0-9]?$ ]] || (( qemu_seconds > 12 )); then
  echo "REI_QEMU_TIMEOUT must be an integer from 1 to 12 seconds" >&2
  exit 2
fi

out=build/qemu/raspi3b
mkdir -p "$out"
flags=(--target=aarch64-none-elf -O1 -ffreestanding -fno-builtin
  -fno-stack-protector -fno-pic -fno-pie -fno-unwind-tables
  -fno-asynchronous-unwind-tables -Werror -Iinclude -Iplatform/raspi3b)

clang "${flags[@]}" -c platform/raspi3b/boot.S -o "$out/boot.o"
clang "${flags[@]}" -c platform/raspi3b/entry.c -o "$out/entry.o"
clang "${flags[@]}" -c platform/raspi3b/uart.c -o "$out/uart.o"
clang "${flags[@]}" -c src/core/rv64-branch.c -o "$out/thistle-branch.o"

ld.lld -m aarch64elf -T platform/raspi3b/linker.ld \
  -o "$out/kernel8.elf" \
  "$out/boot.o" "$out/entry.o" \
  "$out/uart.o" "$out/thistle-branch.o"
llvm-objcopy -O binary "$out/kernel8.elf" "$out/kernel8.img"

echo "AArch64 Raspberry Pi 3B smoke image: $out/kernel8.img"

if $build_only; then
  exit 0
fi

# A file-backed UART is independent of the invoking shell's terminal.
# Keep QEMU diagnostics separate from bytes sent by the guest.
: > "$out/serial.log"
: > "$out/qemu.log"

set +e
timeout -s TERM -k 2s "$qemu_seconds"s \
  qemu-system-aarch64 \
    -M raspi3b \
    -accel tcg,thread=single \
    -display none \
    -monitor none \
    -serial "file:$out/serial.log" \
    -kernel "$out/kernel8.img" \
    </dev/null > "$out/qemu.log" 2>&1
status=$?
set -e

cat "$out/serial.log"
cat "$out/qemu.log" >&2
if (( status != 0 && status != 124 )); then
  echo "QEMU exited unexpectedly with status $status" >&2
  exit 1
fi
grep -Fq 'REI: RASPI3B AARCH64 BOOT' "$out/serial.log" || {
  echo "Raspberry Pi 3B UART boot marker missing (QEMU status $status; see $out/serial.log and $out/qemu.log)" >&2
  exit 1
}
grep -Fq 'REI: THISTLE GENERATED CORE PASS' "$out/serial.log" || {
  echo "Generated Thistle core did not pass under QEMU" >&2
  exit 1
}
if grep -Fq 'REI: THISTLE GENERATED CORE FAIL' "$out/serial.log"; then
  echo "Generated Thistle core explicitly failed" >&2
  exit 1
fi
echo "PASS: QEMU raspi3b booted AArch64 and ran Croissant-generated Thistle code"
