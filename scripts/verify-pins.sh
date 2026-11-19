#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

for dependency in thistle bake croissant; do
  case "$dependency" in
    thistle) label=Thistle ;;
    bake) label=Bake ;;
    croissant) label=Croissant ;;
  esac

  pinned="$(git ls-tree HEAD "$dependency" | awk '$1 == "160000" && $2 == "commit" { print $3 }')"
  if [[ ! "$pinned" =~ ^[a-f0-9]{40}$ ]]; then
    echo "Missing pinned Git submodule: $dependency" >&2
    exit 1
  fi

  if ! grep -Fq "| $label | $pinned |" docs/provenance.md; then
    echo "Provenance mismatch: $dependency has gitlink $pinned" >&2
    exit 1
  fi

  if ! grep -Fq "path = $dependency" .gitmodules; then
    echo "Missing .gitmodules path: $dependency" >&2
    exit 1
  fi

  if [[ -e "$dependency/.git" ]]; then
    checked="$(git -C "$dependency" rev-parse HEAD)"
    if [[ "$checked" != "$pinned" ]]; then
      echo "Checked out $dependency at $checked, expected $pinned" >&2
      exit 1
    fi
  fi

  printf '%-10s %s\n' "$dependency" "$pinned"
done
