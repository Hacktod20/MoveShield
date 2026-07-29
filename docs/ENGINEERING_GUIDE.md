# MoveShield Engineering Kickoff Guide

**Version:** 1.0.0

**Status:** Implementation Begins

---

# Introduction

Welcome to the MoveShield project.

If you are reading this document, the architecture phase has been completed.

This repository is no longer in the design stage.

From this point forward, our goal is to transform the architecture into a production-quality engineering platform.

This document explains the vision, engineering principles, implementation order, development standards, repository organization, and expectations for every contributor.

It is the first document every developer should read before contributing.

---

# What is MoveShield?

MoveShield is an engineering intelligence platform for the Move programming language and the Sui ecosystem.

MoveShield is **not**:

* A compiler
* A package manager
* A formatter
* A linter replacement
* A formal verifier

MoveShield is a static analysis platform that helps developers understand, secure, and improve Move codebases.

Our objective is to become the engineering companion to the official Move toolchain.

Developers should continue using the official tools for compilation and testing.

MoveShield adds analysis, engineering insight, security, architecture visualization, reporting, and developer guidance.

---

# Vision

Our long-term vision is simple.

We want MoveShield to become the standard engineering platform for Move development.

One engine should power multiple products:

* CLI
* GitHub Action
* VS Code Extension
* Language Server
* Cloud Dashboard
* Enterprise Platform
* REST API
* AI Assistant

The engine should be written once.

Every product should reuse it.

---

# Engineering Philosophy

Before writing code, every contributor must understand the engineering philosophy.

## Principle 1

Never duplicate knowledge.

If information has already been computed,

reuse it.

---

## Principle 2

Every crate has one responsibility.

Large crates eventually become impossible to maintain.

Small focused crates are easier to understand.

---

## Principle 3

Everything is immutable whenever practical.

Immutable structures make concurrent execution easier.

They also eliminate entire categories of bugs.

---

## Principle 4

The compiler owns compilation.

MoveShield owns analysis.

We never attempt to replace the official compiler.

Instead, we integrate with it.

---

## Principle 5

Facts are different from findings.

Example:

Complexity = 18

This is a fact.

Whether that is acceptable depends on policy.

Policies transform facts into findings.

---

## Principle 6

Every expensive computation happens once.

Call graphs

Ownership graphs

Dependency graphs

Capability graphs

Knowledge indexes

These should be built once.

Rules reuse them.

---

## Principle 7

Architecture before optimization.

Correctness before speed.

Performance improvements should never reduce correctness.

---

# High-Level Architecture

```
Move Source

↓

Official Sui / Move Toolchain

↓

Compiler Adapter

↓

Core Domain Model

↓

MoveIR

↓

Graph Engine

↓

Knowledge Store

↓

Rule Engine

↓

Policy Engine

↓

Finding Engine

↓

Report Engine

↓

CLI / GitHub / IDE / API
```

Every layer has exactly one responsibility.

No layer bypasses another.

---

# Repository Structure

```
moveshield/

apps/
    cli/

crates/
    moveshield-core
    moveshield-compiler
    moveshield-ir
    moveshield-graph
    moveshield-knowledge
    moveshield-rules
    moveshield-policy
    moveshield-findings
    moveshield-report
    moveshield-cache
    moveshield-config
    moveshield-sdk
    moveshield-testing
    moveshield-utils

docs/

examples/

test-data/

benchmarks/
```

Each crate should remain independently testable.

---

# Development Order

We do **not** implement everything simultaneously.

Implementation proceeds in layers.

## Stage 1

Workspace creation

* Rust workspace
* Cargo configuration
* Formatting
* Clippy
* CI
* Documentation

Deliverable:

Repository builds successfully.

---

## Stage 2

Core crate

Implement

Identifiers

Diagnostics

Errors

Shared traits

Shared models

Result:

Every other crate compiles.

---

## Stage 3

Compiler integration

Goals

Project discovery

Move.toml loading

Dependency resolution

Compiler invocation

Output:

AnalysisSession

Nothing else.

---

## Stage 4

MoveIR

Convert compiler output into a stable semantic representation.

This is where MoveShield begins.

No rules exist yet.

---

## Stage 5

Graph Engine

Implement

Call Graph

Dependency Graph

Ownership Graph

Capability Graph

Graphs should be immutable after construction.

---

## Stage 6

Knowledge Store

Convert graph information into reusable engineering knowledge.

Knowledge contains facts only.

No warnings.

No findings.

---

## Stage 7

Rule Engine

Implement the rule scheduler.

Register the first rule.

Execute it.

Produce observations.

Nothing more.

---

## Stage 8

Policy Engine

Map observations into findings.

Organization policies belong here.

---

## Stage 9

Finding Engine

Normalize findings.

Deduplicate.

Assign metadata.

Prepare reporting.

---

## Stage 10

Report Engine

Generate

Terminal

JSON

SARIF

Markdown

HTML (later)

---

# MVP

The first public release should remain intentionally small.

Supported command:

```
moveshield analyze
```

Supported output:

* Terminal
* JSON
* SARIF

Supported graph:

* Call Graph

Supported rule count:

1

One excellent rule is worth more than fifty incomplete rules.

---

# Coding Standards

Every pull request should satisfy the following.

Formatting

```
cargo fmt
```

Linting

```
cargo clippy
```

Testing

```
cargo test
```

No warnings.

No undocumented public APIs.

No TODOs in merged production code without linked issues.

---

# Testing Requirements

Every feature must include

Unit tests

Integration tests

Regression tests

Snapshot tests (where applicable)

Every bug fix introduces a regression test.

---

# Pull Request Guidelines

Every pull request should answer:

1. What problem does this solve?

2. Why is this approach correct?

3. Which RFC does it implement?

4. What tests were added?

5. Does it introduce a breaking change?

Small focused pull requests are preferred.

---

# Performance

Performance matters.

Correctness matters more.

We optimize only after measurement.

Benchmark before changing algorithms.

Never guess.

---

# Security

MoveShield analyzes security-sensitive software.

Security requirements include:

* No unsafe Rust unless justified and reviewed.
* Validate all external inputs.
* Avoid panics in user-facing paths.
* Preserve deterministic analysis.
* Treat compiler output as trusted only after validation.

---

# Documentation

Every public type.

Every public function.

Every public trait.

Must include documentation.

Documentation is part of the implementation.

---

# First Milestone

The first engineering milestone is considered complete when the following workflow succeeds:

```
moveshield analyze ./example

↓

Project Discovery

↓

Compiler Adapter

↓

MoveIR

↓

Call Graph

↓

MS001 Rule

↓

Finding

↓

SARIF Report

↓

Terminal Output
```

At this point, MoveShield has proven its complete architecture end-to-end.

Everything after that is iteration.

---

# Success Criteria

The project is successful when:

* A developer can analyze a Move project with a single command.
* The architecture remains modular and maintainable.
* New rules can be added without modifying the engine.
* Reports integrate with modern developer workflows.
* The platform grows through reusable crates rather than tightly coupled code.

---

# Final Note

MoveShield is intended to become long-term infrastructure for the Move ecosystem.

Every design decision should favor clarity, correctness, maintainability, and extensibility over short-term convenience.

When in doubt, prefer simple, composable solutions that can evolve with the Move ecosystem rather than tightly coupling the project to any specific compiler implementation or blockchain release.
