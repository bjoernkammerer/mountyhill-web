#!/usr/bin/env bash
set -euo pipefail

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "wasm-pack fehlt: cargo install wasm-pack" >&2
  exit 1
fi

echo "== viz nach wasm =="
wasm-pack build viz --release --target web --out-dir ../dist/wasm --out-name viz
rm -f dist/wasm/.gitignore dist/wasm/package.json dist/wasm/README.md

echo "== seiten =="
cargo run --release -p site -- build

echo
echo "fertig. vorschau: cargo run --release -p site -- serve"
