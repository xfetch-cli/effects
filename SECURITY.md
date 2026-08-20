# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in the **effects ecosystem** (e.g. an effect that
breaks terminal state, emits malformed escape sequences, or hangs xfetch), please report it
responsibly by contacting:

**Email:** `x@xscriptor.com`

### What to Include

When reporting a security issue, please provide:

1. **Description** — A clear explanation of the vulnerability
2. **Type** — What kind of security issue is it? (e.g., ANSI injection, terminal escape, denial of service, supply-chain)
3. **Steps to Reproduce** — Detailed steps to trigger the vulnerability
4. **Impact** — How severe is the issue? What could an attacker do?
5. **Affected Versions** — Which effect and xfetch versions are affected?
6. **Proposed Fix** (optional) — If you have a suggestion for how to fix it

### Guidelines

- **Do not** open public GitHub issues for security vulnerabilities
- **Do not** disclose the vulnerability publicly until a fix is released
- **Do** give the maintainers reasonable time to address the issue before public disclosure
- Typically, we aim to respond within **7 days** and release a fix within **30 days** for critical issues

## Scope

Effects are user-installed binaries (`xfetch effects install`) that xfetch runs to animate the
output before the final fetch is drawn. Anything that lets an effect break out of that role is
in scope:

- **Terminal escape**: effects must never emit partial or crafted ANSI sequences — a malformed
  escape can make the terminal interpret subsequent input (keyboard/control-char injection).
  This is enforced by `xfetch-effects-lib`'s ANSI-safe tokenizer; bypassing it is a
  vulnerability.
- **Availability**: every effect must wrap its work in `with_timeout`; one that can hang the
  fetch indefinitely is a denial-of-service vector.
- **Input handling**: effects must settle on the final (unmodified) content — frames that alter
  or drop rendered data are in scope.
- The `xfetch-effect-api` wire protocol and the build/install path (supply chain).
