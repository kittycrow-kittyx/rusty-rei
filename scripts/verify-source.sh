#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
test -s Cargo.toml
test -s crust.manifest.json
grep -Fxq '#![no_std]' src/lib.rs
grep -Fxq '#![forbid(unsafe_code)]' src/lib.rs
sha256sum -c SHA256SUMS
test -z "$(git ls-files -- dist platform target '*.o' '*.a' '*.so' '*.elf' '*.img' '*.bin' '*.wasm' '*.rlib')"
expected="$(mktemp)"
trap 'rm -f "$expected"' EXIT
{ find src -type f; printf '%s\n' Cargo.toml crust.manifest.json docs/bake-manifest.json docs/provenance.md; } | LC_ALL=C sort > "$expected"
cut -d ' ' -f 3- SHA256SUMS | LC_ALL=C sort | cmp - "$expected"
