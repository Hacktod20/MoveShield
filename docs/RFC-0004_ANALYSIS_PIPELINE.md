# RFC-0004

# Analysis Pipeline

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

This RFC defines the complete analysis pipeline used by MoveShield.

The analysis pipeline transforms a Move project into engineering findings.

Unlike a compiler pipeline, whose purpose is to produce executable bytecode, the MoveShield pipeline exists to produce engineering knowledge.

Its final product is not executable code.

Its final product is developer confidence.

---

# Motivation

Move already provides

• Parsing

• Type Checking

• Bytecode Generation

• Bytecode Verification

MoveShield should never repeat those responsibilities.

Instead,

MoveShield consumes compiler output and enriches it with engineering analysis.

This separation reduces maintenance while allowing the project to evolve alongside the official Move ecosystem.

---

# Pipeline Overview

The pipeline consists of nine stages.

```

Move Project

↓

Project Discovery

↓

Compiler Adapter

↓

Core Data Model

↓

Relationship Builder

↓

Analysis Engine

↓

Rule Engine

↓

Finding Engine

↓

Reporting Engine

---

# Stage 1

Project Discovery

Purpose

Locate a valid Move project.

Responsibilities

Locate

Move.toml

Resolve

Workspace

Dependencies

Source Files

Compiler Configuration

Edition

Target Chain

Expected Output

A complete project description.

---

# Stage 2

Compiler Adapter

Purpose

Delegate compilation to the official Move toolchain.

Responsibilities

Compile packages.

Collect diagnostics.

Read compiler metadata.

Read package metadata.

Return normalized objects.

MoveShield never parses source code directly.

Compilation belongs to the compiler.

---

# Stage 3

Core Data Model Construction

Purpose

Convert compiler output into MoveShield objects.

Compiler

↓

Project

↓

Packages

↓

Modules

↓

Functions

↓

Structs

↓

Resources

↓

Capabilities

↓

Events

↓

Relationships

After this stage,

compiler-specific objects disappear.

Everything else operates on the CDM.

---

# Stage 4

Relationship Builder

Compiler output describes syntax.

MoveShield must understand engineering relationships.

This stage constructs reusable graphs.

Call Graph

Function A

↓

Calls

↓

Function B

Import Graph

Module

↓

Imports

↓

Module

Capability Graph

Capability

↓

Protects

↓

Function

Ownership Graph

Object

↓

Owned By

↓

Address

These graphs are cached for reuse.

---

# Stage 5

Metadata Enrichment

The compiler knows syntax.

MoveShield adds engineering metadata.

Examples

Function Complexity

Documentation Coverage

Visibility Risk

Ownership Classification

Security Classification

Upgrade Risk

Dependency Metadata

Engineering Score

No original compiler data is modified.

Metadata is additive.

---

# Stage 6

Analysis Engine

Purpose

Prepare information required by rules.

Examples

Detect unreachable code.

Compute fan-in.

Compute fan-out.

Compute dependency depth.

Compute object relationships.

Compute module boundaries.

The Analysis Engine performs expensive calculations once.

Rules reuse these results.

---

# Stage 7

Rule Engine

Purpose

Evaluate engineering rules.

Every rule receives

Project

Graphs

Metadata

Relationships

Rules never communicate directly with the compiler.

Every rule remains isolated.

Rule Categories

Security

Architecture

Maintainability

Documentation

Performance

Style

Compatibility

Future

Gas Optimization

AI Recommendations

---

# Rule Execution Model

Rules execute independently.

Rule

↓

Finding

Rules never modify the project.

Rules never communicate with one another.

This enables

Parallel execution.

Predictable behaviour.

Simple testing.

---

# Stage 8

Finding Engine

Purpose

Normalize rule output.

Every rule produces Findings.

Finding

Identifier

Rule

Severity

Confidence

Category

Message

Explanation

Location

Suggested Fix

References

The Finding Engine validates and standardizes all results before reporting.

---

# Stage 9

Reporting Engine

Purpose

Generate output formats.

Supported Outputs

Terminal

JSON

SARIF

Markdown

HTML

Future

PDF

Cloud Dashboard

Reports consume Findings only.

Reports never inspect compiler objects.

Reports never execute rules.

---

# Incremental Analysis

Future versions should avoid reanalyzing unchanged code.

Pipeline

Modified Files

↓

Dependency Analysis

↓

Affected Modules

↓

Affected Rules

↓

Updated Findings

Only affected sections of the project should be recomputed.

---

# Parallel Execution

The architecture should support parallel execution.

Relationship Building

↓

Parallel Rule Execution

↓

Finding Aggregation

This improves scalability on large projects.

---

# Error Model

Compiler Error

Compilation failed.

Pipeline stops.

Analysis Error

Internal processing failed.

Rule Error

One rule failed.

Other rules continue.

Report Error

Output generation failed.

Findings remain available.

Errors are isolated.

---

# Performance Goals

One compilation per scan.

Graph construction performed once.

Relationship cache reused.

Rules remain read-only.

No unnecessary cloning.

Minimal memory allocations.

---

# Future Pipeline

Future stages may include

AI Explanation

Historical Comparison

Cross Repository Analysis

Enterprise Policies

Execution Cost Analysis

Upgrade Simulation

These stages should extend the pipeline rather than replace it.

---

# Architectural Guarantees

The pipeline guarantees

Compiler independence.

Deterministic execution.

Stable interfaces.

Read-only analysis.

Parallel rule execution.

Reusable engineering metadata.

Predictable outputs.

---

# Success Criteria

The pipeline succeeds when

Each stage has a single responsibility.

Compiler changes affect only the adapter.

Rules remain isolated.

Reports remain format-independent.

Performance scales with project size.

Future extensions require minimal architectural change.

---

# Conclusion

The Analysis Pipeline is the backbone of MoveShield.

It transforms compiler output into actionable engineering knowledge through a series of deterministic, modular, and reusable stages.

By separating compilation, enrichment, analysis, rule evaluation, and reporting, MoveShield can evolve independently of the Move compiler while providing developers with fast, explainable, and trustworthy feedback throughout the development lifecycle.