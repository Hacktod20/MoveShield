# RFC-0019

# Testing Strategy

Status:
Draft

Version:
0.1.0

Target:
Entire MoveShield Platform

---

# Abstract

This RFC defines the testing philosophy, infrastructure, and requirements for MoveShield.

Testing is considered a first-class engineering discipline.

Every subsystem, rule, graph, and report format must be verifiable through deterministic automated tests.

No feature is considered complete until it has corresponding automated tests.

---

# Motivation

MoveShield analyzes software that may secure millions or billions of dollars.

False positives reduce trust.

False negatives create risk.

Engine regressions affect every user.

A comprehensive testing strategy is therefore essential for correctness, stability, and long-term maintainability.

---

# Testing Philosophy

MoveShield follows four principles.

Correctness before performance.

Deterministic results.

Fast developer feedback.

Regression prevention.

---

# Testing Pyramid

```

                  End-to-End

               Integration Tests

              Snapshot Tests

             Rule Validation

            Graph Validation

          Component Tests

          Unit Tests

```

Every level contributes to overall confidence.

---

# Test Categories

Version One includes

Unit Tests

Integration Tests

Rule Tests

MoveIR Tests

Graph Tests

Knowledge Store Tests

Policy Tests

Report Tests

CLI Tests

Snapshot Tests

Regression Tests

Performance Benchmarks

Compatibility Tests

---

# Unit Tests

Purpose

Verify individual functions.

Examples

Hashing

Serialization

Rule utilities

Graph algorithms

Configuration parsing

Expected execution time

Milliseconds.

---

# Integration Tests

Purpose

Verify communication between crates.

Examples

Compiler Adapter

↓

MoveIR

↓

Graph Engine

↓

Rule Engine

↓

Finding Engine

Every stage is validated together.

---

# Rule Tests

Every rule owns an isolated test suite.

Each rule includes

Positive examples

Negative examples

Edge cases

Regression cases

Performance limits

Rules are never tested only through end-to-end execution.

---

# MoveIR Tests

Validate

Node generation

Semantic normalization

Relationships

Stable identifiers

Incremental updates

Serialization

Every compiler upgrade must preserve MoveIR correctness.

---

# Graph Tests

Every graph requires

Traversal correctness

Cycle detection

Reachability

Ownership propagation

Capability propagation

Serialization

Incremental rebuild

Large project benchmarks

---

# Snapshot Tests

Many outputs remain stable.

Examples

SARIF

JSON

Markdown

Terminal

MoveIR

Graph exports

Snapshots detect unintended changes.

Intentional changes require snapshot updates during review.

---

# Regression Tests

Every fixed bug produces a permanent regression test.

A bug without a regression test is considered incomplete.

Regression suites grow continuously.

---

# Corpus Tests

MoveShield maintains a public corpus of Move projects.

Categories

Small examples

Real-world Sui projects

Open-source packages

Known vulnerable examples

Known secure examples

Large repositories

Every release runs against the entire corpus.

---

# Compatibility Tests

Every supported compiler version requires

Compilation

MoveIR generation

Rule execution

SARIF generation

Report generation

Compatibility matrices are maintained per release.

---

# Performance Tests

Metrics include

Analysis duration

Memory usage

Graph generation

Rule throughput

Cache hit rate

Incremental analysis time

Performance regressions block release until reviewed.

---

# Fuzz Testing

Future releases should include fuzz testing for

Configuration parser

MoveIR parser

Graph construction

Serialization

Rule inputs

The objective is resilience against malformed input.

---

# Golden Files

Expected outputs are stored as golden files.

Examples

Expected SARIF

Expected JSON

Expected reports

Expected graph exports

Golden files improve review quality and simplify regression detection.

---

# Test Data

Repository layout

```

test-data/

fixtures/

rules/

graphs/

reports/

regressions/

performance/

compatibility/

```

Test data is version controlled.

---

# Continuous Integration

Every pull request executes

Formatting

Linting

Unit Tests

Integration Tests

Snapshot Validation

Rule Tests

Performance Smoke Tests

Documentation Validation

Build Verification

No merge occurs without a successful CI run.

---

# Coverage

Coverage is monitored continuously.

Priority

Core Engine

MoveIR

Graph Engine

Rule Engine

Policy Engine

Coverage percentages are informative rather than the sole quality metric.

---

# Release Validation

Every release candidate executes

Complete corpus

Performance benchmarks

Compatibility suite

Regression suite

SARIF validation

Cross-platform testing

Only successful candidates become releases.

---

# Developer Workflow

Recommended workflow

Write failing test.

Implement feature.

Run local tests.

Open pull request.

CI validates.

Review.

Merge.

---

# Future Extensions

Differential testing

Mutation testing

Distributed benchmarks

Real-world telemetry replay

Cloud-scale validation

Automated rule certification

---

# Success Criteria

The testing strategy succeeds when

every subsystem is independently verifiable,

regressions are detected automatically,

performance remains measurable,

and every release demonstrates repeatable correctness.

---

# Conclusion

Testing is a foundational capability of MoveShield rather than a final development phase.

A disciplined testing strategy enables MoveShield to evolve rapidly while maintaining developer trust, architectural stability, and engineering quality.