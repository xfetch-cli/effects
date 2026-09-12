#!/usr/bin/env bash
# Local CI for the WebAssembly effect examples.
#
# Toolchains that are missing are skipped with a notice; the script fails only
# when an available toolchain cannot build its example.
set -euo pipefail
cd "$(dirname "$0")/.."

if rustup target list --installed 2>/dev/null | grep -q '^wasm32-wasip1$'; then
  echo "==> cargo build --release --target wasm32-wasip1 (wasm-matrix)"
  cargo build --release --target wasm32-wasip1 -p xfetch-effect-wasm-matrix
else
  echo "==> skip Rust wasm (wasm-matrix): wasm32-wasip1 target not installed"
fi

if command -v componentize-py >/dev/null 2>&1; then
  echo "==> componentize-py (wasm-python-pulse)"
  (
    cd effects/wasm-python-pulse
    mkdir -p dist
    componentize-py --quiet -d ../../../api/wit -w effect componentize app -p . -o dist/wasm-python-pulse.wasm
  )
else
  echo "==> skip Python component: componentize-py not in PATH"
fi

echo "==> wasm effects CI OK"
