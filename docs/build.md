# Building Rusty Rei

## Compile the snapshot

From a normal checkout, install the Rust targets and build:

```sh
rustup target add x86_64-unknown-none aarch64-unknown-none
bash scripts/build.sh
```

The checked-in Rust files and Cargo manifest are sufficient. Submodules, Node.js, npm and Bun are not needed. The script builds both targets with warnings denied and at most two jobs, or accepts either target triple:

```sh
bash scripts/build.sh aarch64-unknown-none
```

Libraries are written to `target/<triple>/release/libcrust_kernel.rlib`. A downstream platform supplies the generated `Platform` trait implementation and links an image. Rusty Rei itself defines no boot protocol, board support or image layout.

## Regenerate the snapshot

Install Git, Bun, Node.js and npm and obtain access to the pinned compiler repositories.

```sh
git submodule update --init --recursive
bash scripts/regenerate.sh
bash scripts/build.sh
```

Bake lowers the pinned canonical Thistle source and Crust emits Rust. GitHub Actions regenerates and uploads the source snapshot and checks that it matches the committed files. Compilation outputs remain local. Review and commit the regenerated source, Cargo manifest and compiler manifest together. Do not edit generated kernel modules independently.

## CI and release targets

Both workflows call `.github/workflows/targets.yml` to read crust's current
main-branch compiler workflow and the target contract used by its final job.
They compile the resulting matrix, failing if the upstream contract is missing
or incompatible. Release archives contain the exact policy commit, matrix and
profile; the publishing job checks the complete matrix before publishing.
The target policy is independent of the pinned source-generation dependencies.
