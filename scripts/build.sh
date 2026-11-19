#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
case "${1:-all}" in
  all) targets=(x86_64-unknown-none aarch64-unknown-none) ;;
  x86_64-unknown-none|aarch64-unknown-none) targets=("$1") ;;
  *) echo "Usage: $0 [all|x86_64-unknown-none|aarch64-unknown-none]" >&2; exit 2 ;;
esac
if [[ ! -f Cargo.toml || ! -f src/lib.rs ]]; then
  echo "Generated source snapshot is missing; run scripts/regenerate.sh explicitly" >&2
  exit 1
fi
grep -Fxq '#![no_std]' src/lib.rs
grep -Fxq '#![forbid(unsafe_code)]' src/lib.rs
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-D warnings"
for target in "${targets[@]}"; do
  nice -n 10 cargo build --release -j 2 --target "$target" --lib
  test -s "target/$target/release/libcrust_kernel.rlib"
done
