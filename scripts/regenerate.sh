#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
if [[ -n "$(git status --porcelain --untracked-files=all -- src Cargo.toml crust.manifest.json docs/bake-manifest.json docs/provenance.md SHA256SUMS)" ]]; then
  echo "Refusing to overwrite modified generated source" >&2
  exit 1
fi
case "${1:-}" in
  "") git submodule update --init --recursive
      bash scripts/verify-pins.sh
      bash scripts/fetch-upstream.sh
      bun install --cwd build/upstream/crust --frozen-lockfile
      bun install --cwd build/upstream/crust/bake --frozen-lockfile ;;
  --source-only) bash scripts/verify-pins.sh ;;
  *) echo "Usage: $0 [--source-only]" >&2; exit 2 ;;
esac
for dep in thistle crust; do
  [[ "$(cat "build/upstream/$dep/RELEASE_COMMIT")" == "$(git -C "$dep" rev-parse HEAD)" ]]
done
[[ "$(cat build/upstream/crust/bake/RELEASE_COMMIT)" == "$(git -C crust/bake rev-parse HEAD)" ]]
rm -rf "build/rei" "build/upstream/thistle/build/bake-rust"
bun build/upstream/crust/src/thistle-project.ts build/upstream/thistle
nice -n 10 bun build/upstream/crust/bake/src/cli.ts --project build/upstream/thistle/tsconfig.crust.json \
  --out-dir build/bake-rust --target=rust --engine=host
nice -n 10 bun build/upstream/crust/src/cli.ts \
  --manifest build/upstream/thistle/build/bake-rust/bake-manifest.json --out-dir build/rei
test -s build/rei/Cargo.toml
test -s build/rei/crust.manifest.json
grep -Fxq '#![no_std]' build/rei/src/lib.rs
grep -Fxq '#![forbid(unsafe_code)]' build/rei/src/lib.rs
rm -rf src
cp -R build/rei/src .
cp build/rei/Cargo.toml build/rei/crust.manifest.json .
mkdir -p docs
cp build/upstream/thistle/build/bake-rust/bake-manifest.json docs/bake-manifest.json
{
  printf '# Source provenance\n\n| Component | Git revision |\n|---|---|\n'
  printf '| Thistle | %s |\n' "$(git -C thistle rev-parse HEAD)"
  printf '| Bake | %s |\n' "$(git -C crust/bake rev-parse HEAD)"
  printf '| Crust | %s |\n' "$(git -C crust rev-parse HEAD)"
  printf '\nGenerated source uses the verified releases in upstream.releases.json. Bake is inherited through Crust.\n'
} > docs/provenance.md
{ find src -type f; printf '%s\n' Cargo.toml crust.manifest.json docs/bake-manifest.json docs/provenance.md; } | LC_ALL=C sort | xargs sha256sum > SHA256SUMS
sha256sum -c SHA256SUMS
bash scripts/verify-pins.sh
