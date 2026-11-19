# Architecture

Rei is the native execution target for the canonical Thistle kernel.

```text
thistle/ (pinned canonical TypeScript)
  |
bake/ --target=clang (pinned lowering)
  |
croissant/ (pinned compiler)
  |
src/ and include/ (generated, committed snapshots)
  |
GCC / Clang
  |
dist/ (native relocatable objects)
  |
native intrinsics, platform integration and linker (not yet implemented)
  |
Rei executable (not yet bootable)
```

Thistle owns the kernel implementation. Rei must never keep an
independently edited C equivalent of that implementation. Croissant
generates one C source and a corresponding header per lowered module.
The include path `include/croissant/` names the generator's ABI/header
namespace; the actual compiler checkout is the Git submodule
`croissant/`.

The three Git submodule gitlinks pin exactly the source revisions used
for this snapshot. The revisions are recorded in
[provenance.md](provenance.md) and verified by
`scripts/verify-pins.sh` during CI.

The native platform layer, boot entry and linker scripts will belong
to Rei itself. They are distinct from generated Thistle kernel modules.
