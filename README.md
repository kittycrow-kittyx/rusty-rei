# Rusty Rei

**REI**: Rei Executes Independently.

Rusty Rei is a platform-independent freestanding Rust kernel generated from
Thistle. The generated crate is committed in `src/`.

## Build

Requires Rust and Cargo.

```sh
rustup target add x86_64-unknown-none aarch64-unknown-none
bash scripts/build.sh
```

The build produces libraries for both targets. The platform supplies the
intrinsic trait implementation and links its image separately.

## Source generation

The pinned Thistle, Bake and Crust revisions define the source snapshot.
See [build instructions](docs/build.md) for regeneration and
[provenance](docs/provenance.md) for the exact revisions.

## Licence

[MIT](LICENSE).
