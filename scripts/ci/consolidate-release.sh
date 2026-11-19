#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
TAG=v2.2.0
OLD_RELEASE_COMMIT=b70c169a4577e3b2218a6ca96936aec1e29a43f1
OLD_TAG_OBJECT=dbb82064e72ab9f0d2c1853315d749b087008f4b
RELEASE_PARENT=5f79526b1bd7e14146c629f9184871e745e73812
RELEASE_ID=451408959
[[ "$GH_REPO" == "kittycrow-kittyx/rusty-rei" ]]
test "$(git rev-parse HEAD)" = "$SOURCE_SHA"
test "$(git ls-remote origin refs/heads/main | cut -f1)" = "$SOURCE_SHA"
object="$(git ls-remote --refs origin "refs/tags/$TAG" | cut -f1)"
if [[ "$object" != "$OLD_TAG_OBJECT" ]]; then
  echo "Original release tag is absent or already consolidated."
  exit 0
fi
if [[ "$(git rev-parse HEAD^)" != "$RELEASE_PARENT" ]]; then
  echo "Source is outside the authorised consolidation."
  exit 0
fi
test "$(cat VERSION)" = 2.2.0
gh api "repos/$GH_REPO/actions/runs/$VALIDATION_RUN_ID" | jq -e --arg sha "$SOURCE_SHA" --arg repo "$GH_REPO" '.name == "Kernel source and build validation" and .head_sha == $sha and .head_branch == "main" and .head_repository.full_name == $repo and .event == "push" and .status == "completed" and .conclusion == "success"'
test "$(git rev-parse "$OLD_TAG_OBJECT^{commit}")" = "$OLD_RELEASE_COMMIT"
git diff --quiet "$OLD_RELEASE_COMMIT" "$SOURCE_SHA" -- src Cargo.toml crust.manifest.json SHA256SUMS docs/bake-manifest.json docs/provenance.md VERSION .gitmodules project.dependencies.json upstream.releases.json thistle crust
# No implementation or upstream input changes are authorised by this consolidation.
while IFS= read -r -d '' path; do
  case "$path" in
    .gitattributes|scripts/*|.github/workflows/release.yml) ;;
    *) echo "Unexpected release consolidation path: $path" >&2; exit 1 ;;
  esac
done < <(git diff --name-only -z "$OLD_RELEASE_COMMIT" "$SOURCE_SHA")
stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/original" "$stage/verified"
gh api "repos/$GH_REPO/releases/tags/$TAG" > "$stage/release.json"
jq -e --argjson id "$RELEASE_ID" '.id == $id and .immutable != true and .draft == false' "$stage/release.json"
gh release download "$TAG" --repo "$GH_REPO" --pattern '*.tar.gz' --pattern SHA256SUMS --dir "$stage/original"
(cd "$stage/original" && sha256sum -c SHA256SUMS)
archives=("rusty-rei-v2.2.0-amd64.tar.gz" "rusty-rei-v2.2.0-arm64.tar.gz" "rusty-rei-v2.2.0-armv7.tar.gz" "rusty-rei-v2.2.0-i686.tar.gz" "rusty-rei-v2.2.0-riscv32.tar.gz" "rusty-rei-v2.2.0-riscv64.tar.gz")
test "$(jq '.assets | length' "$stage/release.json")" = "7"
for name in "${archives[@]}" SHA256SUMS; do
  digest="$(jq -er --arg name "$name" '[.assets[] | select(.name == $name and .state == "uploaded")] | if length == 1 then .[0].digest else error("Missing release asset") end' "$stage/release.json")"
  actual="$(sha256sum "$stage/original/$name" | cut -d ' ' -f1)"
  test "$digest" = "sha256:$actual"
done
mkdir -p "$stage/repacked"
for name in "${archives[@]}"; do
  original="$stage/original/$name"
  unpack="$stage/unpacked"
  check="$stage/check"
  mkdir -p "$unpack" "$check"
  tar -xzf "$original" -C "$unpack"
  test "$(cat "$unpack/RELEASE_COMMIT")" = "$OLD_RELEASE_COMMIT"
  # Reuse every compiled library byte and change only archive provenance.
  printf '%s\n' "$SOURCE_SHA" > "$unpack/RELEASE_COMMIT"
  tar --sort=name --mtime="@$(git show -s --format=%ct "$OLD_RELEASE_COMMIT")" \
    --owner=0 --group=0 --numeric-owner -czf "$stage/repacked/$name" -C "$unpack" .
  tar -xzf "$stage/repacked/$name" -C "$check"
  diff -r "$unpack" "$check"
  test "$(cat "$check/RELEASE_COMMIT")" = "$SOURCE_SHA"
  rm -rf "$unpack" "$check"
done
(cd "$stage/repacked" && sha256sum "${archives[@]}" > SHA256SUMS && sha256sum -c SHA256SUMS)
# Recheck the exact release and leased tag before replacing assets in place.
test "$(git ls-remote origin refs/heads/main | cut -f1)" = "$SOURCE_SHA"
test "$(git ls-remote --refs origin "refs/tags/$TAG" | cut -f1)" = "$OLD_TAG_OBJECT"
gh api "repos/$GH_REPO/releases/tags/$TAG" | jq -e --argjson id "$RELEASE_ID" '.id == $id and .immutable != true'
gh release upload "$TAG" "$stage/repacked/"*.tar.gz "$stage/repacked/SHA256SUMS" --clobber --repo "$GH_REPO"
gh release download "$TAG" --repo "$GH_REPO" --pattern '*.tar.gz' --pattern SHA256SUMS --dir "$stage/verified"
(cd "$stage/verified" && sha256sum -c SHA256SUMS)
for name in "${archives[@]}" SHA256SUMS; do cmp "$stage/repacked/$name" "$stage/verified/$name"; done
test "$(git ls-remote origin refs/heads/main | cut -f1)" = "$SOURCE_SHA"
git -c user.name=github-actions -c user.email=41898282+github-actions[bot]@users.noreply.github.com tag -fa "$TAG" "$SOURCE_SHA" -m "Rusty Rei $TAG"
git push --force-with-lease="refs/tags/$TAG:$OLD_TAG_OBJECT" origin "refs/tags/$TAG"
test "$(git ls-remote origin "refs/tags/$TAG^{}" | cut -f1)" = "$SOURCE_SHA"
gh api "repos/$GH_REPO/releases/tags/$TAG" | jq -e --argjson id "$RELEASE_ID" '.id == $id'
