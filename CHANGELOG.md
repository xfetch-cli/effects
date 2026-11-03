# Changelog

## 2026-11-03 — v1.0.2

Release train v1.0.2 (catalog schema 1, catalog_version 1.0.2).

### Dependency updates

- Updated the workspace lockfile with semver-compatible patches: `serde_json` 1.0.154, `syn` 3.0.9 and `unicode-ident` 1.0.27.
- Updated the security-audit workflow to `actions/checkout` 7.0.1 and the pinned `dtolnay/rust-toolchain` commit.
- No effect code or manifests changed; crate versions remain 0.2.0. The catalog index is unchanged.

### Addon catalog index

- Added root `index.json`, a machine-readable catalog of the four installable effects (`decrypt`, `glitch`, `wasm-matrix`, `wasm-python-pulse`), consumed by `xfetch effects search`.
- Native Rust effects publish their Cargo version and description; wasm guests publish the metadata from their adjacent `xfetch-effect.json`, with `guest_runtime` set to `core` or `component`.
- Added `scripts/generate_catalog.py`, a standard-library-only deterministic generator with `--check` support for CI and local verification.
- No effect code or manifests changed. Crate versions remain 0.2.0.

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
