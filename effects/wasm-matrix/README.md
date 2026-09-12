<div align="center">
  <h1>Wasm Matrix Effect</h1>
  <p>Matrix-style reveal effect compiled to WebAssembly.</p>
</div>

<br>

<div align="center">
  <table>
    <tr>
      <td><strong>Kind</strong></td>
      <td><code>effect</code></td>
    </tr>
    <tr>
      <td><strong>Artifact</strong></td>
      <td><code>xfetch-effect-wasm-matrix.wasm</code> (core module)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td><code>wasm32-wasip1</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td>none (pure compute)</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1 -p xfetch-effect-wasm-matrix</code></pre>

<h2>Install</h2>

<pre><code>xfetch effects install ./effects/wasm-matrix</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "effects": [
    { "plugin": "wasm-matrix", "duration_ms": 900, "fps": 30 }
  ]
}</code></pre>

<p>
  The effect scrambles every visible character with deterministic glyphs and
  resolves the text ahead of the progress curve. The final frame always equals
  the original content, and ANSI escape sequences stay intact.
</p>
