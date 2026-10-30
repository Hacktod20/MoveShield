# MoveShield Project Status

Last verified: 2026-10-30

This report replaces the previous handoff snapshot, which contained outdated
branch, test-count, and feature-status information.

## Current verified state

- The Rust workspace builds in debug and release profiles.
- Formatting, strict Clippy, workspace tests, and CLI integration tests pass.
- The CLI supports `analyze`, `init`, `config validate`, `config show`,
  `doctor`, `rules`, and `explain`.
- Analysis output supports terminal, JSON, Markdown, HTML, and SARIF 2.1.0.
  The default is terminal output plus a SARIF file named `moveshield.sarif`.
- Fifteen built-in rules are registered. Tests exercise representative parser,
  configuration, and CLI behavior; this is not comprehensive validation of
  each rule against real-world contracts.
- A GitHub Actions workflow runs formatting, Clippy, tests, and release builds
  on Linux, macOS, and Windows. Its first hosted run requires a push.
- A tag-triggered GitHub Release workflow builds CLI archives for Linux x86_64,
  macOS x86_64/Apple Silicon, and Windows x86_64 and publishes SHA-256 checksums.

## Important limitations

### Compiler integration

Analysis currently uses `DevelopmentParserAdapter`, a lightweight source
parser. It does not invoke or consume the official Sui Move compiler AST and
does not type-check packages. Its parsing, locations, and findings can be
incomplete or incorrect. A clean scan must not be interpreted as a security
guarantee.

The official Sui build tooling is maintained within the Sui workspace; for
example, [`sui-move-build`](https://github.com/MystenLabs/sui/tree/main/crates/sui-move-build)
uses workspace-local Move/Sui dependencies. The Sui onboarding documentation
shows building a Move package with the Sui toolchain:
[Build and Publish a Package](https://docs.sui.io/getting-started/onboarding/hello-world).
Integrating and validating a real compiler adapter is still the highest
priority before production or security-critical use.

### Analysis and product scope

- The graph and knowledge layers provide a useful prototype, not complete
  semantic, ownership, capability, or dataflow analysis.
- Rules need isolated positive, negative, and edge-case test suites, plus
  evaluation against maintained real-world Move packages.
- The policy engine, persistent cache, SDK, and procedural-macro crates are
  incomplete or skeletal; they are not all wired into the CLI.
- Plugin loading, graph visualization, GitHub Action integration, IDE support,
  cloud, and enterprise features are not implemented.
- GitHub release automation is configured, but a release still requires pushing
  a version tag matching `moveshield-cli` after the workflow has been reviewed.
- The project is dual-licensed MIT OR Apache-2.0; each CLI archive includes
  both license texts.
- `doctor` checks local Rust tools and reports whether Sui is installed; it does
  not install or configure toolchains.

## Local verification

Run these commands from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

Analyze the bundled example:

```bash
cargo run --package moveshield-cli -- analyze ./example
```

Exit code `2` means findings were detected. It is an expected scan result, not
a command crash. Use `--format json` for JSON output, `--format sarif --output
PATH` for a selected SARIF file, and `--format terminal` to skip file output.

## Next milestones

1. Implement and validate an official Sui compiler adapter, with supported
   compiler versions and clear diagnostics.
2. Expand per-rule tests and run a representative corpus of real Move projects;
   verify source locations and SARIF with the official schema/tooling.
3. Improve semantic models and analysis before making security guarantees.
4. Complete policy/suppression behavior, or remove unsupported claims until it
   is implemented.
5. Add incremental caching and plugin support only when their behavior and
   compatibility contracts are defined.
6. Plan GitHub Action and IDE integrations after the analysis engine is
   sufficiently reliable.

The PRD also describes cloud and enterprise products. Those require separate
architecture, deployment destinations, credentials, operational ownership,
and release decisions; they are not part of this local CLI implementation.
