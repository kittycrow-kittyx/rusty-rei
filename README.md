# Rusty Rei

**REI** stands for **Rei Executes Independently**.

Rusty Rei is the platform-independent native Rust kernel generated from canonical Thistle through Bake and Crust. The generated source is committed in `src/`. It contains no C kernel or hardware-specific runtime.

## Build

Only Rust and Cargo are required to compile the committed source.

```sh
rustup target add x86_64-unknown-none aarch64-unknown-none
bash scripts/build.sh
```

The build produces freestanding libraries for both architectures. A downstream platform supplies the intrinsic implementation and links its own image.

See [build instructions](docs/build.md) to regenerate the source and [provenance](docs/provenance.md) for the pinned compiler and kernel revisions.

Source generation runs in GitHub Actions against the pinned toolchain. Compiled libraries, objects and images are not committed.
