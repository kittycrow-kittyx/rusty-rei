#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
for dependency in thistle bake crust; do
  pinned="$(git ls-tree HEAD "$dependency" | awk '$1 == "160000" && $2 == "commit" { print $3 }')"
  [[ "$pinned" =~ ^[a-f0-9]{40}$ ]] || { echo "Missing gitlink: $dependency" >&2; exit 1; }
  grep -Fq "| ${dependency^} | $pinned |" docs/provenance.md
  checked="$(git -C "$dependency" rev-parse --show-toplevel)"
  [[ "$checked" == "$(pwd)/$dependency" ]] || { echo "Uninitialised submodule: $dependency" >&2; exit 1; }
  [[ "$(git -C "$dependency" rev-parse HEAD)" == "$pinned" ]] || { echo "Incorrect revision: $dependency" >&2; exit 1; }
  [[ -z "$(git -C "$dependency" status --porcelain --untracked-files=no)" ]] || { echo "Modified tracked source: $dependency" >&2; exit 1; }
  printf '%-10s %s\n' "$dependency" "$pinned"
done
