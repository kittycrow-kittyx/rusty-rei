#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
pins="$(jq -ce --slurpfile releases upstream.releases.json '
  .dependencies as $pins | $releases[0].dependencies as $releases |
  if ($pins | length) == 2 and ($releases | length) == 2 and
    ($pins | map(.path) | sort) == ["crust","thistle"] and
    all($pins[]; . as $pin |
      .direct == true and .repository == ("kittycrow-kittyx/" + .path) and
      (.commit | test("^[0-9a-f]{40}$")) and
      ([$releases[] | select(.path == $pin.path and .repository == $pin.repository and .commit == $pin.commit)] | length) == 1)
  then $pins[] else error("Invalid direct dependency pins") end
' project.dependencies.json)"
while IFS= read -r item; do
  path="$(jq -er .path <<< "$item")"
  sha="$(jq -er .commit <<< "$item")"
  test "$(git ls-files -s -- "$path" | awk '$1 == "160000" {print $2}')" = "$sha"
  test "$(git -C "$path" rev-parse --show-toplevel)" = "$PWD/$path"
  test "$(git -C "$path" rev-parse HEAD)" = "$sha"
  test -z "$(git -C "$path" status --porcelain --untracked-files=no)"
  printf '%s %s\n' "$path" "$sha"
done <<< "$pins"
nested="$(git -C crust ls-tree HEAD bake | awk '$1 == "160000" && $2 == "commit" {print $3}')"
[[ "$nested" =~ ^[0-9a-f]{40}$ ]]
test "$(git -C crust/bake rev-parse --show-toplevel)" = "$PWD/crust/bake"
test "$(git -C crust/bake rev-parse HEAD)" = "$nested"
test -z "$(git -C crust/bake status --porcelain --untracked-files=no)"
printf 'bake %s\n' "$nested"
