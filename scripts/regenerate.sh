#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

bash scripts/verify-pins.sh
git submodule update --init --recursive
bash scripts/verify-pins.sh

if [[ -n "$(git status --porcelain --untracked-files=no --ignore-submodules=dirty)" ]]; then
  echo "Refusing to regenerate over modifications to tracked files" >&2
  exit 1
fi

command -v bun >/dev/null
command -v node >/dev/null
command -v npm >/dev/null
command -v clang >/dev/null
command -v gcc >/dev/null
command -v cmake >/dev/null
command -v ld >/dev/null

npm install --prefix bake
npm install --prefix croissant
npm run build --prefix croissant

node croissant/dist/src/thistle-project.js thistle
(
  cd bake
  bun src/cli.ts --project ../thistle/tsconfig.croissant.json \
    --out-dir build/croissant-bake --target=clang --engine=host
)

node croissant/dist/src/cli.js \
  --manifest thistle/build/croissant-bake/bake-manifest.json \
  --out-dir build/croissant-output

# Only files declared by the old generated manifest may be deleted.
# Preserve future, separately maintained Rei platform code in src/.
node --input-type=module - build/croissant-output <<'NODE'
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const output = path.resolve(process.argv[2]);
const previous = JSON.parse(fs.readFileSync("croissant.manifest.json", "utf8"));
const generated = JSON.parse(
  fs.readFileSync(path.join(output, "croissant.manifest.json"), "utf8")
);
const valid = (relative) => {
  if (!/^(src|include)\/[A-Za-z0-9_./-]+\.(c|h)$/.test(relative) ||
      relative.split("/").includes("..")) {
    throw new Error("Unsafe generated path: " + relative);
  }
  return relative;
};

for (const relative of [...previous.sources, ...previous.headers]) {
  fs.rmSync(path.join(root, valid(relative)), { force: true });
}
for (const relative of [...generated.sources, ...generated.headers]) {
  const destination = path.join(root, valid(relative));
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(path.join(output, relative), destination);
}

const originalCMake = fs.readFileSync(path.join(output, "CMakeLists.txt"), "utf8");
if (!originalCMake.includes("project(croissant_thistle LANGUAGES C)") ||
    !originalCMake.includes("add_library(croissant_thistle OBJECT")) {
  throw new Error("Unexpected Croissant CMake structure; refusing to rewrite");
}
fs.writeFileSync("CMakeLists.txt", originalCMake
  .replace("project(croissant_thistle LANGUAGES C)", "project(rei LANGUAGES C)")
  .replaceAll("croissant_thistle", "rei_kernel"));
fs.copyFileSync(path.join(output, "croissant.manifest.json"), "croissant.manifest.json");
NODE

cp thistle/build/croissant-bake/bake-manifest.json docs/bake-manifest.json

# Host compilation, including the complete relocatable combined object.
CC=gcc cmake -S . -B build/host -DCMAKE_BUILD_TYPE=Release
cmake --build build/host --parallel
rm -rf dist/host-gcc dist/x86_64 dist/aarch64
mkdir -p dist/host-gcc dist/x86_64 dist/aarch64

object_root=build/host/CMakeFiles/rei_kernel.dir/src
mapfile -d '' -t objects < <(find "$object_root" -type f -name '*.o' -print0 | sort -z)
if (( ${#objects[@]} == 0 )); then
  echo "Host build did not produce object files" >&2
  exit 1
fi

for object in "${objects[@]}"; do
  relative="${object#"$object_root/"}"
  mkdir -p "dist/host-gcc/$(dirname "$relative")"
  cp "$object" "dist/host-gcc/$relative"
done
ld -r -o dist/host-gcc/rei-combined.o "${objects[@]}"

# Cross-compile each module independently.
for arch in x86_64 aarch64; do
  case "$arch" in
    x86_64) triple=x86_64-unknown-none-elf ;;
    aarch64) triple=aarch64-none-elf ;;
  esac
  while IFS= read -r -d '' source; do
    relative="${source#src/}"
    object="dist/$arch/${relative%.c}.o"
    mkdir -p "$(dirname "$object")"
    clang --target="$triple" -std=c11 -O1 -ffreestanding \
      -fno-builtin -fno-stack-protector -Werror -Iinclude \
      -c "$source" -o "$object"
  done < <(find src -type f -name '*.c' -print0 | sort -z)
done

cat > docs/provenance.md <<EOF
# Build provenance

Canonical kernel: Thistle

| Component | Git revision |
|---|---|
| Thistle | $(git -C thistle rev-parse HEAD) |
| Bake | $(git -C bake rev-parse HEAD) |
| Croissant | $(git -C croissant rev-parse HEAD) |

Build timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)

All three dependencies are pinned Git submodules. The generated C source,
headers and native object files are development snapshots, not an
independently maintained kernel and not a linked, bootable Rei image.
EOF

{
  find src include dist -type f
  printf '%s\n' croissant.manifest.json docs/bake-manifest.json
} | LC_ALL=C sort | while IFS= read -r file; do sha256sum "$file"; done > SHA256SUMS

sha256sum -c SHA256SUMS >/dev/null
bash scripts/verify-pins.sh
echo "Rei generated C sources and native object snapshots updated from pinned dependencies"
