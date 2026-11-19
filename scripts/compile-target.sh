#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
target="${1:?target required}"
toolchain="${2:?toolchain required}"
build_std="${3:?build-std policy required}"
[[ "$target" =~ ^[a-zA-Z0-9._-]+$ && "$toolchain" =~ ^[a-zA-Z0-9._-]+$ ]]
args=()
case "$build_std" in
  true) args+=(-Z build-std=core,compiler_builtins) ;;
  false) ;;
  *) exit 2 ;;
esac
mkdir -p "build/native/$target"
log="build/native/$target/cargo-artifacts.jsonl"
export RUSTFLAGS="-D warnings -C panic=abort"
cargo +"$toolchain" build "${args[@]}" --release -j 2 --target "$target" --lib --message-format=json-render-diagnostics | tee "$log"
artifact="$(jq -ers '[.[] | select(.reason == "compiler-artifact" and .target.name == "crust_kernel" and (.target.kind | index("lib"))) | .filenames[] | select(endswith(".rlib"))] | unique | if length == 1 then .[0] else error("Expected one kernel rlib") end' "$log")"
test -s "$artifact"
cp "$artifact" "build/native/$target/libcrust_kernel.rlib"
