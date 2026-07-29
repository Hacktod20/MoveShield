# RFC-0006

# Analysis Engine

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

The Analysis Engine is responsible for transforming a compiled Move project into engineering knowledge.

Unlike the Rule Engine, which evaluates engineering policies, the Analysis Engine performs reusable computations that describe the project.

The Analysis Engine never determines whether something is correct or incorrect.

Instead,

it discovers facts.

Those facts become reusable knowledge consumed by rules.

---

# Why This Exists

Suppose we have

500 rules.

Every rule wants to know

Which function calls another function.

Without an Analysis Engine,

every rule builds its own Call Graph.

500 Call Graphs.

500 expensive computations.

Instead

the Analysis Engine computes it once.

Every rule reuses it.

This dramatically reduces runtime.

---

# Philosophy

Compiler

↓

Syntax

↓

Types

↓

Bytecode

MoveShield

↓

Knowledge

The compiler produces executable information.

MoveShield produces engineering information.

---

# Design Goals

The Analysis Engine should

Compute once.

Cache forever.

Remain deterministic.

Be reusable.

Remain independent of rules.

Scale to enterprise repositories.

---

# Non Goals

The Analysis Engine never

Produces findings.

Assigns severity.

Ranks issues.

Formats output.

Reads configuration.

Uploads telemetry.

---

# High-Level Pipeline

```

Compiler Output

↓

Core Data Model

↓

Analysis Engine

↓

Knowledge Store

↓

Rule Engine

↓

Observations

↓

Finding Builder

↓

Findings

```

---

# Engineering Knowledge

Engineering knowledge is information about the project that is expensive to compute but useful to many rules.

Examples

Call Graph

Dependency Graph

Ownership Graph

Capability Graph

Visibility Graph

Inheritance Relationships (future)

Module Relationships

Control Flow Graph

Data Flow Graph

Symbol References

Complexity Metrics

Documentation Coverage

None of these represent findings.

They are facts.

---

# Knowledge Store

The Analysis Engine writes to a Knowledge Store.

Think of it as

Project Intelligence.

Example

Knowledge Store

```

CallGraph

Complexity

Dependencies

Capabilities

Ownership

Metrics

Imports

Documentation

Statistics

Relationships

```

Every rule receives the same Knowledge Store.

---

# Call Graph Builder

Purpose

Determine which functions invoke other functions.

Produces

Incoming Calls

Outgoing Calls

Recursive Calls

Unused Functions

Call Depth

Future

Dynamic call approximation.

---

# Dependency Analyzer

Produces

Package Dependencies

Module Dependencies

Circular Dependencies

Unused Imports

Dependency Depth

Dependency Risk

---

# Capability Analyzer

Discovers

Capability Creation

Capability Transfer

Capability Usage

Capability Destruction

Capability Leakage

Capability Reachability

This information supports authorization-related rules.

---

# Ownership Analyzer

Discovers

Object Ownership

Transfer Paths

Shared Objects

Immutable Objects

Mutation Points

Ownership Chains

---

# Visibility Analyzer

Produces

Public API Surface

Internal APIs

Friend Relationships

Module Boundaries

Unused Public Functions

---

# Complexity Analyzer

Calculates

Cyclomatic Complexity

Maximum Nesting

Statement Count

Branch Count

Loop Count

Function Length

Average Complexity

Maximum Complexity

Rules interpret these values.

---

# Documentation Analyzer

Discovers

Missing Documentation

Coverage

Broken References

Examples

API Documentation

Documentation Density

---

# Metrics Analyzer

Produces project-wide metrics.

Functions

Modules

Packages

Resources

Capabilities

Events

Average Function Length

Average Module Size

Largest Module

Largest Function

Dependency Count

These metrics power dashboards.

---

# Graph Cache

Every expensive graph is cached.

Call Graph

Dependency Graph

Ownership Graph

Import Graph

Capability Graph

Rules never rebuild graphs.

---

# Parallel Analysis

Independent analyzers execute concurrently.

Call Graph

Ownership

Documentation

Metrics

Visibility

Dependencies

Each analyzer produces knowledge independently.

---

# Observation Model

Rules do not inspect compiler output.

Rules inspect knowledge.

Example

Knowledge

↓

Function Complexity

↓

Observation

↓

Finding Builder

↓

Finding

This separation allows future policy engines.

---

# Incremental Analysis

Future versions should compute only changed knowledge.

Modified Function

↓

Affected Graph

↓

Affected Rules

↓

Affected Findings

This enables near-instant analysis for large repositories.

---

# Testing

Every analyzer requires

Correctness tests

Regression tests

Performance benchmarks

Memory benchmarks

Large project tests

Version compatibility tests

---

# Success Criteria

The Analysis Engine succeeds when

knowledge is computed once,

reused everywhere,

cached efficiently,

and remains independent of policy.

---

# Future Expansion

The Analysis Engine should eventually support

Gas estimation

Upgrade simulation

Historical project evolution

Cross-project dependency analysis

Enterprise governance

AI-assisted architectural summaries

Execution profiling

These additions should extend the engine without changing existing interfaces.

---

# Conclusion

The Analysis Engine is MoveShield's knowledge factory.

It transforms compiler output into reusable engineering intelligence.

By separating computation from policy, MoveShield gains performance, extensibility, and architectural clarity while avoiding duplicated work across hundreds of rules.