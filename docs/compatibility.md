# Platform Compatibility

<h2>SDK compatibility</h2>

<p>
  The effects in this repository target <code>xfetch-effect-api 0.3.0</code>.
  The JSON protocol remains <code>1</code> and the WIT package remains
  <code>xfetch:runtime@0.1.0</code>. Effects built against the previous
  <code>0.2.0</code> SDK remain runtime-compatible.
</p>

<p>
  Platform support per effect. <code>✓</code> = works, <code>✗</code> =
  not supported.
</p>

<table>
  <thead>
    <tr>
      <th>Effect</th>
      <th>Linux</th>
      <th>macOS</th>
      <th>Windows</th>
      <th>Notes</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>decrypt</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Pure computation over the rendered lines; no OS dependencies.</td>
    </tr>
    <tr>
      <td><code>glitch</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Pure computation over the rendered lines; no OS dependencies.</td>
    </tr>
    <tr>
      <td><code>wasm-matrix</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>WebAssembly core module; requires a wasm-enabled xfetch build.</td>
    </tr>
    <tr>
      <td><code>wasm-python-pulse</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>WebAssembly component (componentize-py); requires a wasm-enabled xfetch build.</td>
    </tr>
  </tbody>
</table>

<h2>Legend</h2>

<ul>
  <li><code>✓</code> — works on the platform.</li>
  <li><code>✗</code> — not supported; the effect is skipped and xfetch renders normally.</li>
</ul>
