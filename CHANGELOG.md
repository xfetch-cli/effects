# Changelog

## 2026-10-24 — Dependency security automation

- Added `.github/workflows/security-audit.yml`: runs `cargo audit` 0.22.2 with `--deny warnings` on push, pull request and a weekly schedule, with actions pinned by commit SHA and no persisted checkout credentials.
- Added weekly Dependabot updates for Cargo dependencies and GitHub Actions (`open-pull-requests-limit: 5`, minor/patch grouped).
- No effect code or assets changed. The current lockfile audits clean (16 crates, 0 vulnerabilities, 0 warnings).

## 2026-09-12 — WebAssembly effects

- Added `wasm-matrix` (Rust core module): deterministic Matrix-style glyph reveal that settles on the original lines with ANSI sequences intact.
- Added `wasm-python-pulse` (Python component): line-by-line reveal with a block-gradient cursor, built with `componentize-py`.
- Added `scripts/ci-wasm.sh` for local builds of both examples.


## 2026-08-19

- Initial release: `decrypt` and `glitch` effects, plus the shared `xfetch-effects-lib` (ANSI-safe tokenizer + reveal helpers) that both effects build on.
