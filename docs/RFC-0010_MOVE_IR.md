# RFC-0010

# Move Intermediate Representation (MoveIR)

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

This RFC introduces MoveIR,
the canonical engineering intermediate representation used internally by MoveShield.

MoveIR sits between the Core Data Model and the Analysis Engine.

Its purpose is not compilation.

Its purpose is engineering analysis.

MoveIR transforms high-level compiler structures into a simplified semantic model optimized for security analysis, architecture analysis, dependency analysis, and future AI-assisted reasoning.

---

# Motivation

The official Move compiler exists to produce executable bytecode.

MoveShield exists to understand software.

These goals are fundamentally different.

Compiler structures are optimized for correctness.

Engineering analysis requires representations optimized for reasoning.

Therefore,

MoveShield introduces MoveIR.

---

# Why Another Representation?

Suppose one rule wants

Call Graphs.

Another wants

Capability propagation.

Another wants

Ownership tracking.

Another wants

Resource mutation.

If every rule traverses compiler objects,

analysis becomes

slow,

complex,

duplicated,

and tightly coupled.

MoveIR becomes the shared engineering language.

---

# Pipeline

Move Source

↓

Move Compiler

↓

Compiler AST

↓

Core Data Model

↓

MoveIR

↓

Knowledge Engine

↓

Policies

↓

Reports

---

# Design Goals

MoveIR must be

Stable

Compiler Independent

Serializable

Deterministic

Immutable

Graph Friendly

Incrementally Updatable

Extensible

---

# Principles

MoveIR represents

behavior,

relationships,

flows,

ownership,

capabilities,

rather than syntax.

---

# Core Objects

MoveProject

MovePackage

MoveModule

MoveFunction

MoveStruct

MoveResource

MoveObject

MoveCapability

MoveEvent

MoveConstant

MoveType

MoveReference

Every object has

Stable ID

Location

Relationships

Metadata

---

# Semantic Nodes

Unlike compiler nodes,

MoveIR introduces semantic nodes.

Examples

Transfer

Borrow

Mutate

Publish

Destroy

Share

Freeze

Call

Return

Abort

Emit Event

Acquire Resource

Release Resource

These become reusable building blocks.

---

# Semantic Edges

Nodes are connected using edges.

CALLS

USES

CREATES

TRANSFERS

MUTATES

BORROWS

PROTECTS

EMITS

RETURNS

DEPENDS_ON

Every analysis reuses the same graph.

---

# Graph Model

MoveIR internally represents projects as a directed graph.

Nodes

↓

Semantic entities.

Edges

↓

Engineering relationships.

This enables

path traversal,

impact analysis,

dependency visualization,

security tracing,

and architectural reasoning.

---

# Ownership Graph

MoveIR explicitly models ownership.

Object

↓

Owner

↓

Transfer

↓

Mutation

↓

Destruction

Ownership becomes queryable.

---

# Capability Graph

Capabilities become first-class entities.

Capability

↓

Creator

↓

Consumers

↓

Protected Functions

↓

Transfer Paths

↓

Revocation

---

# Call Graph

Functions become graph nodes.

Edges represent

calls,

recursive calls,

cross-package calls,

framework calls.

Future versions may include indirect call approximation.

---

# Data Flow

MoveIR should support

source,

propagation,

sink,

allowing future taint analysis.

---

# Control Flow

Future versions may attach

basic blocks,

branches,

loops,

dominators,

to enable deeper analyses.

---

# Query API

MoveIR exposes graph queries.

Examples

Find every public entry point.

Locate every capability consumer.

Find ownership paths.

Find recursive call chains.

Detect cyclic dependencies.

Rules use queries instead of manually traversing structures.

---

# Serialization

MoveIR supports

JSON

Binary

Snapshot

Incremental Cache

This enables offline analysis and future cloud services.

---

# Incremental Updates

When a file changes,

only affected graph regions should be rebuilt.

Unaffected nodes remain cached.

---

# AI Readiness

MoveIR should expose a stable semantic representation suitable for AI systems.

Rather than reading raw syntax,

future AI assistants consume MoveIR.

This provides consistent reasoning regardless of compiler changes.

---

# Comparison

Compiler AST

Optimized for parsing.

MoveIR

Optimized for engineering.

Knowledge Store

Optimized for reusable facts.

Policies

Optimized for organizational decisions.

Each layer has a single responsibility.

---

# Success Criteria

MoveIR succeeds when

new analyses require graph queries rather than compiler traversal,

compiler upgrades require only adapter updates,

and engineering features share one semantic model.

---

# Conclusion

MoveIR becomes the semantic backbone of MoveShield.

It separates syntax from engineering meaning,
allowing security analysis,
architecture visualization,
AI tooling,
and future platform capabilities to evolve independently of the compiler.