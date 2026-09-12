# Changelog

## 2026-09-12 — WebAssembly effects

- Added `wasm-matrix` (Rust core module): deterministic Matrix-style glyph reveal that settles on the original lines with ANSI sequences intact.
- Added `wasm-python-pulse` (Python component): line-by-line reveal with a block-gradient cursor, built with `componentize-py`.
- Added `scripts/ci-wasm.sh` for local builds of both examples.


## 2026-08-19

- Initial release: `decrypt` and `glitch` effects, plus the shared `xfetch-effects-lib` (ANSI-safe tokenizer + reveal helpers) that both effects build on.
