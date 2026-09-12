<div align="center">
  <h1>Wasm Python Pulse Effect</h1>
  <p>Python WebAssembly effect built with <code>componentize-py</code>.</p>
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
      <td><code>wasm-python-pulse.wasm</code> (component)</td>
    </tr>
    <tr>
      <td><strong>Runtime</strong></td>
      <td>Component model, world <code>effect</code></td>
    </tr>
    <tr>
      <td><strong>Capabilities</strong></td>
      <td>none (<code>log</code> only)</td>
    </tr>
    <tr>
      <td><strong>Toolchain</strong></td>
      <td><code>componentize-py</code> &gt;= 0.25</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>python3 -m venv .venv
. .venv/bin/activate
pip install -r requirements.txt
componentize-py -d ../../../api/wit -w effect componentize app -p . -o dist/wasm-python-pulse.wasm</code></pre>

<h2>Install</h2>

<pre><code>xfetch effects install ./effects/wasm-python-pulse</code></pre>

<h2>Configuration</h2>

<pre><code class="language-jsonc">{
  "effects": [
    { "plugin": "wasm-python-pulse", "duration_ms": 800, "fps": 30 }
  ]
}</code></pre>

<p>
  The effect reveals one line at a time and ends with a block-gradient cursor.
  The last frame is exactly the rendered content, so chaining with other
  effects is seamless.
</p>
