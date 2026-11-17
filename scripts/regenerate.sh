#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
git submodule update --init --recursive
bash scripts/verify-pins.sh
if [[ -n "$(git status --porcelain --untracked-files=no -- src Cargo.toml crust.manifest.json)" ]]; then
  echo "Refusing to overwrite modified generated source" >&2
  exit 1
fi
nice -n 10 npm install --prefix bake
nice -n 10 npm install --prefix crust
nice -n 10 npm run build --prefix crust
node crust/dist/src/thistle-project.js thistle
(
  cd bake
  nice -n 10 bun src/cli.ts --project ../thistle/tsconfig.crust.json \
    --out-dir build/bake-rust --target=rust --engine=host
)
nice -n 10 node crust/dist/src/cli.js \
  --manifest thistle/build/bake-rust/bake-manifest.json \
  --out-dir build/rei
test -f build/rei/Cargo.toml
grep -Fxq '#![no_std]' build/rei/src/lib.rs
grep -Fxq '#![forbid(unsafe_code)]' build/rei/src/lib.rs
# The entire src tree is generated; it must contain no platform-owned code.
rm -rf src
cp -R build/rei/src src
cp build/rei/Cargo.toml build/rei/crust.manifest.json .
