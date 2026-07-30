# MoveShield Static Analysis Platform

MoveShield is a production-quality, modular static analysis and engineering intelligence platform for the Move programming language and Sui blockchain ecosystem. Designed from the ground up under `RFC-0020`, MoveShield offers a unidirectional compiler-to-reporting pipeline that analyzes smart contracts, constructs immutability call graphs, extracts structural metrics, and reports architectural or security findings.

---

## 🚀 Key Features

- **Decoupled Architecture:** Built on a modular Rust Cargo workspace with strict dependency hierarchies.
- **Robust Discovery & Adaptation:** Auto-discovers Move projects, resolves named addresses, and leverages a lightweight **Development Parser Fallback** when blockchain toolchains are missing.
- **Advanced Graph Engine:** Assembles a fully resolved, immutable **Call Graph** with recursion and unused function detection.
- **Configurable Policies:** Implements extensible analysis rules, including **MS001 Cyclomatic Complexity**, governed by custom `moveshield.toml` settings.
- **Standardized Reports:** Prints detailed terminal results and generates industry-compliant **SARIF 2.1.0** reports (`./moveshield.sarif`) out-of-the-box.

---

## 📦 Workspace Overview

The platform is split into 14 highly cohesive Cargo crates:

| Crate Name | Primary Responsibility |
| :--- | :--- |
| [`moveshield-core`](crates/moveshield-core) | Canonical Core Data Model (CDM), traits, location info, and standard errors. |
| [`moveshield-compiler`](crates/moveshield-compiler) | Project discovery, Move.toml metadata, and the Development Parser compiler adapter. |
| [`moveshield-ir`](crates/moveshield-ir) | Decoupled semantic intermediate representation for static analysis lookup trees. |
| [`moveshield-graph`](crates/moveshield-graph) | Immutable Call Graph construction, call site resolution, and cycles traversal. |
| [`moveshield-knowledge`](crates/moveshield-knowledge) | Engineering knowledge store housing clean, factual analysis stats. |
| [`moveshield-rules`](crates/moveshield-rules) | The static analysis scheduler and modular rules engine (e.g., MS001). |
| [`moveshield-policy`](crates/moveshield-policy) | Severity overrides, filters, and organizational suppression profiles. |
| [`moveshield-findings`](crates/moveshield-findings) | Deduplicates and deterministically sorts platform findings. |
| [`moveshield-report`](crates/moveshield-report) | Outputs detailed terminal tables and exports standardized `moveshield.sarif`. |
| [`moveshield-config`](crates/moveshield-config) | Handles `moveshield.toml` loading, default configuration, and validation. |
| [`moveshield-sdk`](crates/moveshield-sdk) | High-level developer plugin boundaries and SDK interface definitions. |
| [`moveshield-testing`](crates/moveshield-testing) | Shared testing mocks, harness fixtures, and regression pipelines. |
| [`moveshield-cache`](crates/moveshield-cache) | Platform caching mechanics to speed up consecutive local and CI scan executions. |
| [`moveshield-utils`](crates/moveshield-utils) | Standardized, non-business internal utility helpers. |

---

## ⚡ Quick Start

### 1. Build the platform
Verify you have the stable Rust compiler toolchain set up:

```bash
cargo build --release
```

### 2. Run the tests
Execute the entire workspace unit and integration test suite:

```bash
cargo test
```

### 3. Analyze the Example Move Project
Run the compiled CLI binary against the packaged demo:

```bash
cargo run --package moveshield-cli -- analyze ./example
```

This will run project discovery, compile modules under `example/sources`, evaluate the cyclomatic complexity of functions, and output a detailed warning about `example::complex::transfer` because its complexity of **12** exceeds the configured limit of **10**.

---

## 📘 Comprehensive Documentation

Detailed architecture specifications, developer onboarding, rules specifications, and ADR logs can be accessed directly in the `docs/` workspace:

- **[System Architecture & Scan Pipeline](docs/ARCHITECTURE.md):** Deep-dive into overall scan pipeline diagrams, Mermaid flows, CDM details, ADR logs, and guide on how to add custom analysis rules.
- **[Configuration & Developer Setup Guide](docs/DEVELOPER_GUIDE.md):** Detailed reference for `moveshield.toml`, fresh OS/dependency setups, testing guidelines, and troubleshooting.
- **[Founding Principles](docs/0000_FOUNDING_PRINCIPLES.md):** Engineering kickoff guide outlining MoveShield's long-term design visions and architectural milestones.
