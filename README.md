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

The pinned Thistle and Crust releases define the source snapshot. Bake is inherited through Crust. Regeneration uses Bash and Bun 1.4.5 or later to execute upstream TypeScript; native compilation remains independent.
See [build instructions](docs/build.md) for regeneration and
[provenance](docs/provenance.md) for the exact revisions.

## Releases

[GitHub Releases](https://github.com/kittycrow-kittyx/rusty-rei/releases) distributes
freestanding architecture-specific archives for every target validated by
[crust CI's final stage](https://github.com/kittycrow-kittyx/crust/blob/02b88890398adf47bce3e012bf1d9755acbb7770/.github/workflows/compiler.yml).
The [upstream target contract](https://github.com/kittycrow-kittyx/crust/blob/02b88890398adf47bce3e012bf1d9755acbb7770/targets.json)
is read from the pinned compiler release, not duplicated here. The release version
is stored in `VERSION`, independently of generator-owned files.

Each archive is named `rusty-rei-v<version>-<architecture>.tar.gz` and
contains the Rust rlib, generated Rust source and Cargo manifest, compiler
information, source provenance and the MIT licence. Rust rlibs require a compatible compiler; the source is included for rebuilding. The i686 bare-metal target uses nightly Rust and builds core from source.
Each package records the upstream target-policy revision and profile.
These are kernel libraries, not bootable images.

The release workflow compiles the committed source without regenerating it,
checks the ELF architecture, and publishes the complete discovered set plus
`SHA256SUMS` only after every target succeeds. Successful validation of a push to `main` triggers the release for that exact commit and creates the tag matching `VERSION`. A manual workflow run builds archives without publishing. Use a new `VERSION` for subsequent releases.

## Licence

[MIT](LICENSE).
