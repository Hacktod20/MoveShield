# RFC-0007

# Knowledge Store

Status:
Draft

Version:
0.1.0

Authors:
Muiz

Target:
MoveShield Core

---

# Abstract

The Knowledge Store is the central repository of engineering intelligence inside MoveShield.

Its responsibility is simple.

Store facts.

Not opinions.

Not warnings.

Not findings.

Only facts.

Every subsystem in MoveShield consumes knowledge from the Knowledge Store.

Nothing writes directly to reports.

Nothing writes directly to findings.

Everything first becomes engineering knowledge.

The Knowledge Store is therefore the heart of the platform.

---

# Motivation

Suppose ten different rules want to know whether a function is recursive.

Without a Knowledge Store:

Rule A computes recursion.

Rule B computes recursion.

Rule C computes recursion.

Rule D computes recursion.

Eventually every rule duplicates expensive work.

This wastes CPU time.

It also introduces inconsistent implementations.

Instead,

MoveShield computes engineering knowledge once.

Every subsystem shares the same result.

---

# Design Goals

The Knowledge Store must satisfy the following principles.

• Immutable after construction

• Fast lookups

• Compiler independent

• Deterministic

• Serializable

• Extensible

• Memory efficient

• Thread safe

---

# Philosophy

Knowledge is not a finding.

Example

Function

transfer()

Complexity

12

This is knowledge.

It is neither good nor bad.

Later,

a policy may decide

Complexity > 10

↓

Warning

The Knowledge Store never performs that evaluation.

---

# High Level Architecture

Compiler

↓

Core Data Model

↓

Analysis Engine

↓

Knowledge Store

↓

Policy Engine

↓

Observations

↓

Findings

Knowledge always exists before policy.

---

# Categories of Knowledge

The Knowledge Store contains multiple domains.

Structural Knowledge

Relationship Knowledge

Ownership Knowledge

Capability Knowledge

Complexity Knowledge

Documentation Knowledge

Dependency Knowledge

Visibility Knowledge

Performance Knowledge

Statistics

Future AI Metadata

---

# Structural Knowledge

Contains information about the shape of a project.

Project

Packages

Modules

Functions

Structs

Resources

Events

Constants

Named Addresses

Compiler Metadata

No engineering interpretation exists here.

---

# Relationship Knowledge

Relationships are expensive to compute.

MoveShield stores them once.

Examples

Function Calls

Module Imports

Package Dependencies

Friend Relationships

Object Ownership

Capability Protection

Inheritance (future)

Cross Package References

---

# Capability Knowledge

Every discovered capability is represented explicitly.

Capability Type

Creation Points

Usage Points

Transfer Paths

Consumers

Protected Functions

Reachability

Rules never reconstruct capability graphs.

---

# Ownership Knowledge

Ownership analysis stores

Object Ownership

Shared Objects

Mutable References

Immutable References

Transfer Chains

Mutation Locations

Lifecycle Metadata

---

# Complexity Knowledge

Stores engineering metrics.

Cyclomatic Complexity

Maximum Nesting

Loop Count

Branch Count

Statement Count

Function Size

Average Module Complexity

Project Complexity

Rules decide whether values are acceptable.

---

# Documentation Knowledge

Stores

Coverage

Missing Documentation

Broken Links

Examples

API Comments

Public Documentation Percentage

---

# Dependency Knowledge

Contains

Imported Packages

Version Information

Dependency Depth

Circular Dependencies

Unused Imports

Dependency Graph

Future

Known Vulnerabilities

License Metadata

Compatibility Metadata

---

# Visibility Knowledge

Stores

Public APIs

Internal APIs

Friend APIs

Private Members

Unused Public Functions

Boundary Information

---

# Statistics

Project level statistics.

Modules

Functions

Resources

Capabilities

Events

Lines of Code

Average Complexity

Dependency Count

Documentation Coverage

Analysis Duration

Statistics power dashboards and trends.

---

# Query Model

The Knowledge Store exposes a read-only API.

Example

Knowledge

↓

Lookup

↓

Result

Rules never mutate stored knowledge.

---

# Caching

Knowledge is cached for the lifetime of one analysis session.

Future versions may persist cache across executions to support incremental analysis.

Cache invalidation must occur whenever source code affecting a knowledge domain changes.

---

# Thread Safety

The Knowledge Store is shared across concurrent rule execution.

Therefore

all structures must be immutable,

or internally synchronized where mutation is unavoidable.

The preferred design is immutable data.

---

# Serialization

The Knowledge Store should support

JSON

Binary

Future Snapshot Format

This enables

Debugging

Testing

Benchmarking

Incremental Analysis

Historical Comparison

---

# Extension Model

Future analyzers may register additional knowledge domains.

Example

Gas Analysis

↓

Gas Knowledge

Upgrade Analysis

↓

Upgrade Knowledge

Enterprise Policy

↓

Compliance Knowledge

AI Engine

↓

Recommendation Knowledge

The architecture must permit expansion without breaking existing APIs.

---

# Memory Model

One Knowledge Store exists per analysis session.

All rules receive shared references.

Knowledge objects are never copied unnecessarily.

Large graphs should be stored once and referenced throughout the analysis pipeline.

---

# Versioning

Each knowledge domain includes a schema version.

Future additions should extend schemas without breaking compatibility whenever possible.

Breaking schema changes require a major version increment.

---

# Success Criteria

The Knowledge Store succeeds when

every expensive computation is performed once,

every rule consumes the same engineering facts,

new analyzers can add knowledge without modifying existing components,

and compiler changes never leak into higher layers.

---

# Conclusion

The Knowledge Store transforms MoveShield from a collection of independent analyses into a coherent engineering platform.

It establishes a single source of truth for engineering intelligence, allowing policies, rules, reports, and future services to operate on consistent, reusable knowledge.

By separating facts from interpretation, the Knowledge Store becomes one of the foundational architectural components upon which the entire MoveShield ecosystem is built.