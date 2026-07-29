# RFC-0020

# Workspace Architecture

Status:
Accepted

Version:
1.0.0

Target:
Entire MoveShield Platform

---

# Abstract

This RFC defines the repository structure, crate boundaries, dependency rules,
versioning strategy, ownership model, release process, and architectural
constraints of the MoveShield project.

The workspace architecture is designed for long-term maintainability.

Every crate has one responsibility.

Every dependency is intentional.

Every public API is stable.

---

# Vision

MoveShield is not a CLI.

MoveShield is not a scanner.

MoveShield is an engineering platform.

The CLI is only one application built on top of the platform.

Future products include

CLI

VS Code Extension

Language Server

GitHub Action

Cloud Dashboard

Enterprise Edition

Web API

Documentation Generator

AI Assistant

Every one of those products shares the same core crates.

---

# Design Principles

The workspace follows these principles.

Single Responsibility

Explicit Dependencies

Stable Public APIs

Incremental Compilation

Independent Testing

Minimal Coupling

Maximum Reuse

---

# Repository Layout

```
moveshield/

├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── SECURITY.md
├── CHANGELOG.md
│
├── apps/
│   ├── cli/
│   ├── github-action/
│   ├── language-server/
│   └── cloud/
│
├── crates/
│   ├── moveshield-core/
│   ├── moveshield-ir/
│   ├── moveshield-compiler/
│   ├── moveshield-graph/
│   ├── moveshield-knowledge/
│   ├── moveshield-rules/
│   ├── moveshield-policy/
│   ├── moveshield-findings/
│   ├── moveshield-report/
│   ├── moveshield-cache/
│   ├── moveshield-config/
│   ├── moveshield-sdk/
│   ├── moveshield-utils/
│   ├── moveshield-testing/
│   └── moveshield-macros/
│
├── docs/
│   ├── rfcs/
│   ├── architecture/
│   ├── guides/
│   └── decisions/
│
├── examples/
├── benchmarks/
├── test-data/
├── scripts/
└── .github/
```

---

# Crate Responsibilities

## moveshield-core

Shared domain models.

Traits.

Identifiers.

Diagnostics.

Errors.

Common interfaces.

No compiler logic.

---

## moveshield-compiler

Integration with

Sui

Move

Compiler Adapter

Workspace Discovery

Manifest loading

No rule logic.

---

## moveshield-ir

MoveIR.

Semantic model.

Node construction.

Normalization.

Stable identifiers.

---

## moveshield-graph

Graph construction.

Call Graph.

Dependency Graph.

Ownership Graph.

Capability Graph.

Traversal APIs.

---

## moveshield-knowledge

Engineering knowledge.

Metrics.

Facts.

Indexes.

Query APIs.

---

## moveshield-rules

Rule execution.

Rule registry.

Rule metadata.

Rule scheduler.

Rule lifecycle.

---

## moveshield-policy

Policy loading.

Severity mapping.

Organization profiles.

Overrides.

Suppression.

---

## moveshield-findings

Finding models.

Observation models.

Evidence.

Recommendations.

Deduplication.

---

## moveshield-report

Terminal.

SARIF.

JSON.

Markdown.

HTML.

Future exporters.

---

## moveshield-cache

Incremental analysis.

Persistent cache.

Hashes.

Serialization.

---

## moveshield-config

Configuration loading.

Validation.

Profiles.

Environment variables.

CLI overrides.

---

## moveshield-sdk

Public APIs.

Plugin interfaces.

Extension traits.

Version contracts.

---

## moveshield-testing

Fixtures.

Snapshots.

Regression helpers.

Corpus loader.

Benchmark helpers.

---

## moveshield-utils

Internal reusable utilities.

No business logic.

---

## moveshield-macros

Procedural macros.

Future derive helpers.

Compile-time code generation.

---

# Dependency Rules

```
apps
 │
 ▼
report
 │
 ▼
findings
 │
 ▼
policy
 │
 ▼
rules
 │
 ▼
knowledge
 │
 ▼
graph
 │
 ▼
ir
 │
 ▼
compiler
 │
 ▼
core
```

Dependencies always point downward.

Lower crates never depend on higher crates.

Circular dependencies are forbidden.

---

# Public API

Only

core

sdk

config

report

are intended as stable public libraries.

Everything else may evolve internally.

---

# Internal APIs

Internal crates expose

pub(crate)

whenever possible.

Minimize exported surface area.

---

# Versioning

Semantic Versioning.

Major

Breaking changes.

Minor

New features.

Patch

Bug fixes.

---

# Feature Flags

Examples

```
sarif

html

graphviz

lsp

cloud

enterprise

benchmarks
```

Optional functionality remains isolated.

---

# Build Order

1

core

2

compiler

3

ir

4

graph

5

knowledge

6

rules

7

policy

8

findings

9

report

10

apps

Every layer depends only on completed lower layers.

---

# Ownership

Every crate has

Maintainer

Documentation

Benchmarks

Tests

Public changelog

Clear ownership improves long-term sustainability.

---

# Testing Rules

Every crate contains

Unit tests

Integration tests

Documentation tests

Benchmark suite

Regression suite

No crate is exempt.

---

# Documentation

Every public API

must contain

Examples

Safety notes

Performance notes

Version notes

Documentation is part of the product.

---

# CI Requirements

Every pull request executes

cargo fmt

cargo clippy

cargo test

snapshot tests

benchmarks (smoke)

documentation validation

SARIF validation

Cross-platform builds

---

# Release Process

Development

↓

Alpha

↓

Beta

↓

Release Candidate

↓

Stable

Every stage requires passing automated validation.

---

# Architectural Constraints

Rules never access compiler internals.

Policies never execute rules.

Reports never modify findings.

Graphs are immutable after construction.

MoveIR is immutable.

Configuration is read-only after startup.

The engine owns execution.

Plugins contribute behavior only.

---

# Success Criteria

The workspace succeeds when

new features are added by extending existing layers,

crates remain independently testable,

public APIs remain stable,

and repository growth does not increase architectural complexity.

---

# Conclusion

The MoveShield workspace architecture establishes the structural foundation of the platform.

By enforcing clear crate boundaries, deterministic dependencies, and disciplined ownership, the project is positioned for long-term evolution while remaining approachable for contributors and reliable for production use.