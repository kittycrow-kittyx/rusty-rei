# Building Rei

## Pinned dependencies

After cloning Rei, initialise its Git submodules:

```sh
git submodule update --init --recursive
bash scripts/verify-pins.sh
```

`thistle/`, `bake/` and `croissant/` are pinned to the precise Git
revisions that produced the initial generated snapshot. Upstream `main`
moving must not silently change a Rei build. Access to the private
Croissant repository is required to initialise that submodule.

## Rebuild committed C

The existing generated sources and native objects are build snapshots.
To compile the currently committed C without regenerating it:

```sh
cmake -S . -B build
cmake --build build --parallel
```

To regenerate from the pinned TypeScript source, use:

```sh
bash scripts/regenerate.sh
```

The regeneration script builds Bake and Croissant, performs
`Bake --target=clang --engine=host`, emits a modular C tree, refreshes
only previously generated `src/` and `include/` files, recompiles
the native objects and updates the snapshot checksums.

Run it on a clean working tree. Review all generated changes before
committing them. Submodule updates and regenerated outputs should be
reviewed together.

## Current boundary

The build produces relocatable object files, not a linked Rei image.
`__bake_c_*` and `croissant_js_*` calls still require a native
runtime implementation, followed by architecture-specific platform
integration and linking.
