# MoveShield Static Analysis Platform

MoveShield is a modular static-analysis prototype for the Move programming language and Sui blockchain ecosystem. It demonstrates a compiler-adapter-to-reporting pipeline that analyzes Move source files, builds a call graph, and reports code-quality and security-pattern findings.

MoveShield is available under either the [MIT License](LICENSE-MIT) or
[Apache License 2.0](LICENSE-APACHE), at your option.

> **Important limitation:** MoveShield currently uses a heuristic development parser, not the official Sui Move compiler. It does not type-check packages, and its findings may be incomplete or incorrect. A clean scan is not evidence that a package is secure. Official compiler-AST integration remains future work.

---

## 🚀 Key Features

- **Decoupled Architecture:** Built on a modular Rust Cargo workspace with strict dependency hierarchies.
- **Robust Discovery & Adaptation:** Auto-discovers Move projects, resolves named addresses, and leverages a lightweight **Development Parser Fallback** when blockchain toolchains are missing.
- **Advanced Graph Engine:** Assembles a fully resolved, immutable **Call Graph** with recursion and unused function detection.
- **Configurable Policies:** Implements extensible analysis rules, including **MS001 Cyclomatic Complexity**, governed by custom `moveshield.toml` settings.
- **Reports:** Prints terminal results and exports JSON, Markdown, HTML, and SARIF 2.1.0 (`./moveshield.sarif` by default).

---

## 📦 Workspace Overview

The workspace contains 15 supporting library crates:

| Crate Name | Primary Responsibility |
| :--- | :--- |
| [`moveshield-core`](crates/moveshield-core) | Canonical Core Data Model (CDM), traits, location info, and standard errors. |
| [`moveshield-compiler`](crates/moveshield-compiler) | Project discovery, Move.toml metadata, and the Development Parser compiler adapter. |
| [`moveshield-ir`](crates/moveshield-ir) | Decoupled semantic intermediate representation for static analysis lookup trees. |
| [`moveshield-graph`](crates/moveshield-graph) | Immutable Call Graph construction, call site resolution, and cycles traversal. |
| [`moveshield-knowledge`](crates/moveshield-knowledge) | Engineering knowledge store housing clean, factual analysis stats. |
| [`moveshield-rules`](crates/moveshield-rules) | The static analysis scheduler and modular rules engine (e.g., MS001). |
| [`moveshield-policy`](crates/moveshield-policy) | Policy pipeline entry point; advanced filters and suppressions remain future work. |
| [`moveshield-findings`](crates/moveshield-findings) | Deduplicates and deterministically sorts platform findings. |
| [`moveshield-report`](crates/moveshield-report) | Outputs detailed terminal tables and exports standardized `moveshield.sarif`. |
| [`moveshield-config`](crates/moveshield-config) | Handles `moveshield.toml` loading, default configuration, and validation. |
| [`moveshield-sdk`](crates/moveshield-sdk) | SDK skeleton for future programmatic integrations. |
| [`moveshield-testing`](crates/moveshield-testing) | Shared testing mocks, harness fixtures, and regression pipelines. |
| [`moveshield-cache`](crates/moveshield-cache) | Cache skeleton; persistent incremental caching is not yet wired into the CLI. |
| [`moveshield-utils`](crates/moveshield-utils) | Standardized, non-business internal utility helpers. |

---

## ⚡ Quick Start

### Download a CLI release

Prebuilt CLI archives are published for Linux x86_64, macOS x86_64, macOS
Apple Silicon, and Windows x86_64 on the
[GitHub Releases page](https://github.com/Hacktod20/MoveShield/releases).
Download the archive matching your system and verify it against `SHA256SUMS`
before extracting it. Each archive contains the `moveshield` executable
(`moveshield.exe` on Windows), README, and license texts. Releases are created
from version tags such as `v0.1.0`, and the tag must match the CLI package
version.

### 1. Build the platform
Verify you have the stable Rust compiler toolchain set up:

```bash
cargo build --release
```

### 2. Check your environment and run the tests

```bash
cargo run --package moveshield-cli -- doctor
cargo test --workspace
```

### 3. Analyze the Example Move Project
Run the compiled CLI binary against the packaged demo:

```bash
cargo run --package moveshield-cli -- analyze ./example
```

This scans modules under `example/sources` and reports findings. The example is expected to exit with code `2` because findings were detected; this is not a build or runtime failure. The development parser does not compile or type-check Move code.

### CLI commands

```bash
cargo run --package moveshield-cli -- rules
cargo run --package moveshield-cli -- explain MS001
cargo run --package moveshield-cli -- config validate ./example
cargo run --package moveshield-cli -- init ./some-move-project

# JSON is written to stdout unless --output is provided.
cargo run --package moveshield-cli -- analyze ./example --format json
cargo run --package moveshield-cli -- analyze ./example --format sarif --output ./moveshield.sarif
cargo run --package moveshield-cli -- analyze ./example --format markdown --output ./report.md
cargo run --package moveshield-cli -- analyze ./example --format html --output ./report.html
cargo run --package moveshield-cli -- analyze ./example --format terminal
```

By default, `analyze` prints the terminal report and writes `moveshield.sarif` to the current directory. Findings return exit code `2`, configuration errors return `3`, and other failures return `1`.

---

## 📘 Comprehensive Documentation

Detailed architecture specifications, developer onboarding, rules specifications, and ADR logs can be accessed directly in the `docs/` workspace:

- **[System Architecture & Scan Pipeline](docs/ARCHITECTURE.md):** Deep-dive into overall scan pipeline diagrams, Mermaid flows, CDM details, ADR logs, and guide on how to add custom analysis rules.
- **[Configuration & Developer Setup Guide](docs/DEVELOPER_GUIDE.md):** Detailed reference for `moveshield.toml`, fresh OS/dependency setups, testing guidelines, and troubleshooting.
- **[Founding Principles](docs/0000_FOUNDING_PRINCIPLES.md):** Engineering kickoff guide outlining MoveShield's long-term design visions and architectural milestones.
