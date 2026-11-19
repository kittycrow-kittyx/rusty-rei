# Rei

**REI** stands for **Rei Executes Independently**.

Rei is the native, freestanding execution target for the canonical
Thistle kernel. The kernel remains implemented in Thistle, not in an
independently maintained C fork.

## Toolchain

The upstream repositories are Git submodules, pinned to the exact revisions
used to produce the committed generated source and native objects:

| Submodule | Purpose |
|---|---|
| `thistle/` | Canonical TypeScript kernel source |
| `bake/` | Lower Thistle for the native Clang target |
| `croissant/` | Convert Bake output into modular, freestanding C |

The headers under `include/croissant/` are **generated C declarations**,
not Croissant's compiler source. The compiler itself is checked out in the
separate, pinned `croissant/` submodule.

## Repository layout

- `src/`: Croissant-generated C kernel source, divided by module.
- `include/`: Croissant-generated C headers and ABI declarations.
- `dist/`: Captured GCC and Clang relocatable ELF objects.
- `docs/`: Architecture, source provenance and build instructions.
- `scripts/`: Provenance checks and regeneration.
- `croissant.manifest.json`: Mapping of Bake modules to generated C files.
- `thistle/`, `bake/`, `croissant/`: Pinned upstream sources.

The generated C and binaries are committed development snapshots. They must
not become an independent source of truth.

## Build

```sh
git submodule update --init --recursive
cmake -S . -B build
cmake --build build --parallel
bash scripts/verify-pins.sh
```

To regenerate the source, manifest, and native object snapshots from the
pinned dependencies, see [Building Rei](docs/build.md).

## ARM64 Raspberry Pi emulation

Rei has a bounded QEMU `raspi3b` smoke test which boots an AArch64
`kernel8.img`, initialises the Raspberry Pi PL011 UART and executes
a Croissant-generated function from the canonical Thistle source.

```sh
bash scripts/qemu-raspi3b-smoke.sh
```

The test stops QEMU automatically and verifies serial output. See
[QEMU Raspberry Pi 3B smoke test](docs/qemu-raspi3b.md) for prerequisites,
the exact hardware model and what this milestone does **not** prove.

## Status

Rei currently compiles the canonical Thistle kernel to separate C
translation units and native relocatable ELF objects on x86-64 and ARM64.
The native intrinsic ABI and platform linker/boot integration remain
unimplemented. Rei is **not** linked or bootable yet.

The immediate milestone is implementing and validating those native
intrinsics before introducing a freestanding platform entry point.
