# Building Rusty Rei

Build the checked-in Rust using Cargo and the appropriate Rust target.

```sh
rustup target add x86_64-unknown-none aarch64-unknown-none
bash scripts/build.sh
```

Regeneration requires Bash, Git, GitHub CLI authentication and Bun 1.4.5 or later. Run `bash scripts/regenerate.sh` to initialise the root pins, verify the published upstream archives, and regenerate the snapshot and checksums. Bake is owned by Crust.

CI transpiles the source, compiles every target from the pinned upstream final validation contract, and checks the generated snapshot against the committed files. Successful main validation triggers the versioned release archives. Platform integrations supply intrinsic implementations and image linking.
