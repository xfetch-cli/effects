<h1 align="center">
<img src="https://raw.githubusercontent.com/xfetch-cli/assets/main/logo/banner/xfetch.svg" width="30%" alt="XFetch banner" />Effects</h1>

<div align="center">

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/xfetch-cli/effects/blob/main/LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-lightgrey?style=flat-square)](https://github.com/xfetch-cli/effects/blob/main/README.md)
[![Rust](https://img.shields.io/badge/Rust-1.97-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

<p>Installable intro animations for <a href="https://github.com/xfetch-cli/xfetch">xfetch</a> — written in Rust.</p>

</div>

<!--Menu-->
<div align="left">
  <h2>Menu</h2>
  <ul>
    <li><a href="#features">Features </a></li>
    <li><a href="#quick-install">Quick Install </a></li>
    <li><a href="#configuration">Configuration </a> </li>
    <li><a href="#writing-an-effect">Writing an Effect </a> </li>
    <li><a href="#structure">Structure </a> </li>
    <li><a href="#related-documents">Related Documents </a>
    </li>
    <li><a href="#related-repos">Related Repos </a> </li>
    <li><a href="#about-the-developer">About X </a> </li>
  </ul>
</div>

<h2 id="features" align="center"> Features</h2>

<p>
  Effects change the way the info appears when xfetch starts: the core renders
  the module lines, hands them to an effect binary (<code>xfetch-effect-&lt;name&gt;</code>),
  and plays the returned frames before settling on the final content.
</p>

<table align="center">
  <thead>
    <tr><th>Effect</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>decrypt</strong></td>
      <td>Reveals each line from scrambled glyphs (smooth decode).</td>
    </tr>
    <tr>
      <td><strong>glitch</strong></td>
      <td>Stuttery scrambled flicker with corruption bursts, horizontal slices and dropped rows.</td>
    </tr>
    <tr>
      <td><strong>wasm-matrix</strong></td>
      <td>Matrix-style glyph reveal, compiled to WebAssembly (Rust).</td>
    </tr>
    <tr>
      <td><strong>wasm-python-pulse</strong></td>
      <td>Line-by-line reveal with a block-gradient cursor (Python component).</td>
    </tr>
  </tbody>
</table>

<h2 id="quick-install" align="center"> Quick Install</h2>

```bash
xfetch effects install <name>     # e.g. decrypt, glitch
```

Or from a local path:

```bash
xfetch effects install ./effects/decrypt
```

Manage installed effects:

```bash
xfetch effects list               # list installed effects
xfetch effects remove <name>      # remove an effect
```

> Effects are opt-in: without the binary installed, xfetch renders normally.
> Use <code>--repo &lt;url&gt;</code> or the <code>XFETCH_EFFECT_REPO</code> env var to install from a fork or mirror.

<h2 id="configuration" align="center"> Configuration </h2>

Add an <code>"effects"</code> block to your <code>config.jsonc</code>. It accepts a
single effect or a list — effects play in sequence.

```jsonc
// Configuration for xfetch
{
  // Effects to play at startup (in order)
  "effects": [
    { "plugin": "glitch", "duration_ms": 700, "fps": 30 },
    { "plugin": "decrypt", "duration_ms": 1500, "fps": 30 }
  ],
  "modules": ["os", "cpu", "memory"]
}
```

| Field         | Type   | Default      | Description                                |
| ------------- | ------ | ------------ | ------------------------------------------ |
| `plugin`      | string | —            | Effect name (binary `xfetch-effect-<name>`). |
| `style`       | string | none         | Effect-specific style selector.            |
| `duration_ms` | number | effect default | Total animation length in ms.            |
| `fps`         | number | effect default | Frames per second.                        |
| `args`        | object | none         | Free-form parameters passed to the effect. |
| `timeout_secs`| number | none         | Safety net: kills the effect if it runs longer. |

<h2 id="writing-an-effect" align="center"> Writing an Effect</h2>

<p>
  Effects speak the <code>xfetch-effect-api</code> protocol (see
  <a href="https://github.com/xfetch-cli/api">xfetch-cli/api</a>): read an
  <code>EffectRequest</code> from stdin (the rendered lines plus
  <code>style</code>/<code>duration_ms</code>/<code>fps</code>/<code>args</code>) and
  write an <code>EffectResponse</code> with a non-empty list of
  <code>{ delay_ms, lines }</code> frames. The last frame should reach the final
  content.
</p>

<p>
  Use the shared <code>xfetch-effects-lib</code> for ANSI-safe text helpers so
  escape sequences (colors/icons) are never broken apart.
</p>

<h2 id="structure" align="center"> Structure</h2>

```
effects/
├── Cargo.toml            # workspace
├── effects-lib/          # shared ANSI-safe helpers (xfetch-effects-lib)
└── <effect-name>/        # one crate per effect (binary xfetch-effect-<name>)
    ├── Cargo.toml
    ├── README.md
    ├── CHANGELOG.md
    ├── configs/          # example config snippets
    └── src/main.rs
```

<h2 id="related-documents" align="center">Related Documents</h2>

<div align="left">
  <ul>
    <li><a href="https://github.com/xfetch-cli/xfetch/blob/main/docs/EFFECTS.md">Effects (xfetch docs)</a></li>
    <li><a href="https://github.com/xfetch-cli/api/blob/main/docs/effect-sdk.md">Effect SDK</a></li>
    <li><a href="https://github.com/xfetch-cli/api/blob/main/docs/protocol.md">Protocol Reference</a></li>
    <li><a href="https://github.com/xfetch-cli/api/blob/main/docs/examples.md">Examples</a></li>
    <li><a href="https://github.com/xfetch-cli/effects/blob/main/LICENSE">License</a></li>
  </ul>
</div>

<p align="center"><em>Contribute to the project, report issues, or connect with the developer using the links around</em></p>

<h2 align="center" id="related-repos">Related Repos</h2>
<ul>
  <li><a href="https://github.com/xfetch-cli/xfetch">XFetch</a></li>
  <li><a href="https://github.com/xfetch-cli/api">XFetch API</a></li>
  <li><a href="https://github.com/xfetch-cli/plugins">XFetch Plugins </a></li>
  <li><a href="https://github.com/xfetch-cli/extensions">XFetch Extensions </a></li>
  <li><a href="https://github.com/xfetch-cli/themes">XFetch Themes </a></li>
  <li><a href="https://github.com/xfetch-cli/configs">XFetch Configs </a></li>
</ul>

<div id="about-the-developer" align="center">
<h2>X</h2>

<a href="https://xscriptor.io">Dev</a>
 & 
<a href="https://github.com/xscriptor">Git</a>
 & 
<a href="https://www.xscriptor.com">X</a>

</div>
