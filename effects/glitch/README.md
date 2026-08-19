# Glitch Effect

Intro effect for xfetch: the content stutters through scrambled glyphs with
random "corruption bursts" before settling on the real text.

## Build

```bash
cargo build --release --manifest-path effects/glitch/Cargo.toml
```

Binary: `effects/glitch/target/release/xfetch-effect-glitch`

## Install

```bash
xfetch effects install glitch
```

## Configuration

```jsonc
{
  "effects": {
    "plugin": "glitch",
    "duration_ms": 700,
    "fps": 30
  }
}
```

| Field         | Type   | Default | Description                 |
| ------------- | ------ | ------- | --------------------------- |
| `plugin`      | string | —       | Effect name (`glitch`).     |
| `duration_ms` | number | `800`   | Total animation length in ms. |
| `fps`         | number | `30`    | Frames per second.          |

`style` and `args` are passed through the protocol and currently unused. Like
all effects, `glitch` is opt-in: without the binary installed, xfetch renders
normally. ANSI escape sequences (colors/icons) are kept intact at all times.

## Chaining

Effects play in sequence. Combine a glitch burst with a clean reveal:

```jsonc
{
  "effects": [
    { "plugin": "glitch", "duration_ms": 700, "fps": 30 },
    { "plugin": "decrypt", "duration_ms": 1000, "fps": 30 }
  ]
}
```

## Protocol

Speaks `xfetch-effect-api`: `EffectRequest` in, `EffectResponse` (non-empty
list of `{ delay_ms, lines }` frames) out. The last frame is the real content.
