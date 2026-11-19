#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
mkdir -p build
stage="$(mktemp -d "$PWD/build/.upstream.XXXXXX")"
trap 'rm -rf "$stage"' EXIT
jq -e '[.dependencies[].path] | sort == ["crust","thistle"]' upstream.releases.json
while IFS= read -r dependency; do
  repo="$(jq -r .repository <<< "$dependency")"
  dir="$(jq -r .path <<< "$dependency")"
  commit="$(jq -r .commit <<< "$dependency")"
  tag="$(jq -r .tag <<< "$dependency")"
  asset="${repo##*/}-$tag.tar.gz"
  [[ "$repo" == "kittycrow-kittyx/$dir" ]]
  [[ "$(git ls-files -s -- "$dir" | awk '$1 == "160000" {print $2}')" == "$commit" ]]
  [[ "$(git -C "$dir" rev-parse HEAD)" == "$commit" ]]
  ref="$(git ls-remote "https://github.com/$repo.git" "refs/tags/$tag^{}" | cut -f1)"
  if [[ -z "$ref" ]]; then ref="$(git ls-remote "https://github.com/$repo.git" "refs/tags/$tag" | cut -f1)"; fi
  [[ "$ref" == "$commit" ]]
  ready=false
  for attempt in {1..12}; do
    download="$(mktemp -d "$stage/.download.XXXXXX")"
    rm -rf "$stage/$dir"
    mkdir -p "$stage/$dir"
    if gh release download "$tag" --repo "$repo" --pattern "$asset" --pattern SHA256SUMS --dir "$download" &&
      (cd "$download" && sha256sum -c SHA256SUMS) &&
      tar -xzf "$download/$asset" -C "$stage/$dir" &&
      [[ "$(cat "$stage/$dir/RELEASE_COMMIT")" == "$commit" ]]; then
      ready=true
      rm -rf "$download"
      break
    fi
    rm -rf "$download"
    if [[ "$attempt" != 12 ]]; then sleep 10; fi
  done
  [[ "$ready" == true ]] || { echo "Verified release not available: $repo $tag at $commit" >&2; exit 1; }
done < <(jq -c '.dependencies[]' upstream.releases.json)
bake="$(git -C crust ls-tree HEAD bake | awk '$1 == "160000" {print $3}')"
[[ "$bake" =~ ^[a-f0-9]{40}$ ]]
[[ "$(cat "$stage/crust/bake/RELEASE_COMMIT")" == "$bake" ]]
jq -e --arg sha "$bake" 'any(.dependencies[]; .path == "bake" and .commit == $sha)' "$stage/crust/upstream.releases.json"
rm -rf build/upstream
mv "$stage" build/upstream
