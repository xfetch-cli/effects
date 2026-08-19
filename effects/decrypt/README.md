# Decrypt Effect

Intro effect for xfetch: reveals the info lines from scrambled glyphs (a
"decrypt" / terminal-scramble look) before settling on the final content.

## Build

```bash
cargo build --release --manifest-path effects/decrypt/Cargo.toml
```

The binary will be created at:

```
effects/decrypt/target/release/xfetch-effect-decrypt
```

## Install

```bash
xfetch effects install decrypt
```

This clones the effects repository, builds the effect and copies
`xfetch-effect-decrypt` into `~/.config/xfetch/effects/`. You can also install
from a local path: `xfetch effects install ./effects/decrypt`.

## Configuration

Add an `"effects"` block to your config. The effect plays when xfetch starts.

```jsonc
{
  "effects": {
    "plugin": "decrypt",
    "duration_ms": 1500,
    "fps": 30
  }
}
```

| Field          | Type   | Default      | Description                          |
| -------------- | ------ | ------------ | ------------------------------------ |
| `plugin`       | string | —            | Effect name (`decrypt`).             |
| `duration_ms`  | number | `1500`       | Total animation length in ms.        |
| `fps`          | number | `30`         | Frames per second.                   |

`style` and `args` are passed through the protocol and currently unused by this
effect. Effects are opt-in: without the binary installed, xfetch renders
normally.

## Protocol

The effect speaks `xfetch-effect-api`:

### Request

```json
{
  "version": 1,
  "kind": "effect",
  "lines": [" X x86_64", " AMD Ryzen 9"],
  "args": { "duration_ms": 1500, "fps": 30 }
}
```

### Response

```json
{
  "frames": [
    { "delay_ms": 33, "lines": ["xeri$5ep#9t4", "#1bnf9c#i"] },
    { "delay_ms": 33, "lines": [" X x86_64", " AMD Ryzen 9"] }
  ]
}
```

Errors are printed to stderr and the process exits non-zero.
