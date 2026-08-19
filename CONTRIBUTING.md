<h1>Contributing Effects</h1>

<p>
  Thanks for contributing to the <strong>xfetch</strong> effects ecosystem.
  This repository contains official effects and the reference implementations
  for building new ones.
</p>

<h2>Workflow</h2>

<ol>
  <li>Fork the repository and create a feature branch.</li>
  <li>Create a new effect crate under <code>effects/&lt;name&gt;/</code>.</li>
  <li>Add it to the workspace <code>members</code> in <a href="./Cargo.toml">Cargo.toml</a>.</li>
  <li>Run <code>cargo test --workspace</code> and <code>cargo build --workspace</code>.</li>
  <li>Document the effect in its own <code>README.md</code> (build, install, config, protocol) and add it to the table in <a href="./README.md">README.md</a>.</li>
  <li>Add a short entry to the effect's <code>CHANGELOG.md</code>.</li>
  <li>Open a pull request with usage details and any required dependencies. PRs that do not compile or fail tests are rejected.</li>
</ol>

<h2>Effect Rules</h2>

<ul>
  <li>Use the binary naming convention <code>xfetch-effect-&lt;name&gt;</code>.</li>
  <li>Keep effects focused on a single visual idea.</li>
  <li>Write errors to stderr and exit with a non-zero status on failure.</li>
  <li>
    <strong>Never break ANSI escape sequences.</strong> The rendered lines carry
    colors and icons; tokenize them with <code>xfetch-effects-lib</code>
    (<code>tokenize</code>/<code>rebuild</code>) and only transform the visible
    characters. A partial escape sequence makes the terminal interpret it and
    leaks garbage onto the screen — this is enforced by the effects-lib tests.
  </li>
  <li>
    <strong>Every effect MUST have a runtime limit.</strong> Wrap all work in
    <code>with_timeout</code> (from <code>xfetch_effect_api</code>) with a
    <code>const BUDGET</code> that fits the work. An effect without a timeout is
    rejected: it could hang xfetch forever.
  </li>
  <li>Prefer deterministic output (seeded) so tests are stable.</li>
  <li>The last frame should reach the final (unmodified) content.</li>
  <li>Prefer stable, actively maintained dependencies and keep them minimal.</li>
</ul>

<h2>Protocol Guide</h2>

<p>
  The wire protocol (<code>xfetch-effect-api</code>) is documented in
  <a href="https://github.com/xfetch-cli/api/blob/main/docs/effect-sdk.md">effect-sdk.md</a>
  and the <a href="https://github.com/xfetch-cli/api/blob/main/docs/protocol.md">protocol reference</a>.
  Shared ANSI-safe helpers live in <code>effects/effects-lib</code> — reuse them
  instead of reimplementing the tokenizer.
</p>

<h2>Code of Conduct</h2>

<p>
  Be respectful, constructive, and collaborative. Harassment, trolling, and
  personal attacks are not tolerated.
</p>
