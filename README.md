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

## Releases

[GitHub Releases](https://github.com/kittycrow-kittyx/rusty-rei/releases) distributes
freestanding architecture-specific archives for every target validated by
[crust CI's final stage](https://github.com/kittycrow-kittyx/crust/blob/main/.github/workflows/compiler.yml).
The [upstream target contract](https://github.com/kittycrow-kittyx/crust/blob/main/targets.json)
is discovered at workflow runtime, not duplicated here. The release version
is stored in `VERSION`, independently of generator-owned files.

Each archive is named `rusty-rei-v<version>-<architecture>.tar.gz` and
contains the Rust rlib, generated Rust source and Cargo manifest, compiler
information, source provenance and the MIT licence. Rust rlibs require a compatible compiler; the source is included for rebuilding. The i686 bare-metal target uses nightly Rust and builds core from source.
Each package records the upstream target-policy revision and profile.
These are kernel libraries, not bootable images.

The release workflow compiles the committed source without regenerating it,
checks the ELF architecture, and publishes the complete discovered set plus
`SHA256SUMS` only after every target succeeds. A manual workflow run
builds the archives without publishing. To publish after validation,
push a version tag matching `VERSION` (currently `v2.2.0`).
Enable immutable releases in repository settings to protect published assets.

## Licence

[MIT](LICENSE).
