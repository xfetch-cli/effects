# Changelog

## 2026-08-19

- Initial release: stuttery scrambled flicker with corruption bursts, horizontal line slices and dropped rows, settling on the real text. Deterministic and ANSI-safe.
- Made the effect more aggressive: per-line independent scrambling, block-glyph alphabet (█ ▓ ▒ ░ ▀ ▄ ▌ ▐ ╬ ╪ ╫), bursts every ~3 frames.
- Built on `xfetch-effects-lib` (shared ANSI-safe helpers) and `with_timeout` with a 10 s budget.
