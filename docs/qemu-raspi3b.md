# Raspberry Pi 3B AArch64 QEMU smoke test

Rei's first Raspberry Pi target is QEMU `raspi3b`, representing a
BCM2837 board with four Cortex-A53 ARM64 cores. This is an actual emulated
Raspberry Pi platform, not QEMU's generic `virt` machine.

## What this milestone proves

The test loads a bare-metal AArch64 `kernel8.img` at `0x80000`.
Rei's **platform-owned** reset assembly sets up a stack and PL011 UART,
then calls `croissant_core_rv64_branch_ts__rv64Branch` from
`src/core/rv64-branch.c`. That function was generated from the pinned
canonical Thistle implementation.

It validates ARM64 instruction execution, a Pi-specific UART register,
calling a generated Thistle C module, and serial output.

This **does not** initialise `tetoKernelInit`, provide the
`__bake_c_*` ABI, start a scheduler, or run an interactive OS.
Rei is not yet a fully bootable kernel.

## Run locally

On Ubuntu/Debian install required tools:

```sh
sudo apt install qemu-system-arm clang lld llvm
```

Then:

```sh
bash scripts/qemu-raspi3b-smoke.sh
```

The test creates its image and serial log beneath
`build/qemu/raspi3b/`. It runs QEMU with a maximum eight-second guest
window and terminates the emulator automatically. A pass requires both
UART markers:

```text
REI: RASPI3B AARCH64 BOOT
REI: THISTLE GENERATED CORE PASS
```

You can shorten the guest runtime with `REI_QEMU_TIMEOUT=4`.
The script rejects durations above 12 seconds to protect developer
machines from indefinite emulation.

## Why Pi 3B

Pi 3B is QEMU-supported, provides ARM64 Cortex-A53 CPUs and emulates
the BCM2837 PL011 UART at physical address `0x3f201000`.
The platform files are under `platform/raspi3b/`, distinct from the
Croissant-generated kernel under `src/`.

Subsequent work will implement a native memory/intrinsic runtime and
link enough canonical Thistle code to invoke kernel initialisation.

To compile the image without starting QEMU, use
`bash scripts/qemu-raspi3b-smoke.sh --build-only`. The smoke test captures
UART bytes directly in `serial.log` and emulator diagnostics in `qemu.log`.
It does not depend on an interactive terminal.
