# Changelog

## 2026-08-19

- Initial release: reveals the info lines from scrambled glyphs (smooth decode), deterministic and ANSI-safe.
- Fixed scramble breaking ANSI escape sequences (partial escapes leaked garbage onto the terminal); escapes are now tokenized and kept intact.
- Refactored to use `xfetch-effects-lib` (shared ANSI-safe helpers) and `with_timeout` with a 10 s budget.
