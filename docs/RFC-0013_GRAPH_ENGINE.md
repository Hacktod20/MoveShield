# RFC-0013

# Graph Engine

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

The Graph Engine is responsible for constructing, maintaining, caching, and querying all engineering graphs used by MoveShield.

It converts semantic relationships from MoveIR into reusable graph structures.

Rules never construct graphs.

Rules query graphs.

The Graph Engine is the only subsystem responsible for graph creation.

---

# Motivation

Every engineering analysis eventually asks questions like

Which functions call this function?

Which capability protects this resource?

Which module imports this package?

Can this object eventually reach this function?

Without reusable graphs,

every rule repeats expensive traversal logic.

The Graph Engine computes relationships once.

Every subsystem reuses them.

---

# Goals

The Graph Engine must

Build graphs once

Cache graph data

Support fast traversal

Support incremental updates

Remain deterministic

Be compiler independent

Scale to enterprise repositories

---

# Non Goals

The Graph Engine does not

Generate findings

Assign severities

Read configuration

Produce reports

Compile projects

Interpret engineering policy

---

# Architecture

MoveIR

↓

Graph Builder

↓

Graph Store

↓

Graph Query API

↓

Rules

---

# Core Graph Types

Version One

Call Graph

Dependency Graph

Import Graph

Ownership Graph

Capability Graph

Visibility Graph

Resource Graph

Event Graph

Future

Control Flow Graph

Data Flow Graph

Taint Graph

Package Graph

Upgrade Graph

Trust Graph

---

# Graph Node

Every graph node contains

Stable Identifier

Kind

Source Location

Metadata

Relationships

Version

Nodes never contain policy.

---

# Graph Edge

Every edge contains

Source

Target

Relationship Type

Direction

Metadata

Strength (future)

Edges are immutable.

---

# Call Graph

Represents function invocation.

Supports

Incoming Calls

Outgoing Calls

Recursive Calls

Maximum Depth

Cross Package Calls

Unused Functions

Entry Points

---

# Dependency Graph

Represents package relationships.

Supports

Direct Dependencies

Transitive Dependencies

Circular Dependencies

Framework Dependencies

Unused Dependencies

Dependency Depth

---

# Ownership Graph

Models Move ownership.

Supports

Ownership Transfer

Shared Objects

Mutation Paths

Object Lifecycles

Ownership Reachability

---

# Capability Graph

Represents authorization.

Supports

Capability Creation

Capability Usage

Capability Consumers

Capability Propagation

Capability Reachability

Protected Functions

---

# Visibility Graph

Models API exposure.

Supports

Public APIs

Friend APIs

Private APIs

Boundary Analysis

Unused Public Members

---

# Event Graph

Represents event flow.

Supports

Emitters

Consumers

Related Resources

Related Modules

---

# Query API

Every graph exposes a read-only query interface.

Example operations

Find callers

Find callees

Find shortest dependency path

Detect cycles

Locate ownership chain

Locate capability path

Rules never traverse raw graph structures directly.

---

# Caching

Graphs are built once per analysis session.

Each graph records

Version

Hash

Generation Time

Source Snapshot

Future versions may persist graphs for incremental analysis.

---

# Incremental Updates

When source changes,

only affected graph regions are rebuilt.

Unaffected nodes remain cached.

This minimizes analysis time.

---

# Thread Safety

Graphs are immutable after construction.

Multiple rules may query graphs concurrently without synchronization overhead.

---

# Serialization

Supported formats

Binary Cache

JSON

Future GraphML

Future DOT

Serialization enables

Testing

Visualization

Cloud Analysis

Incremental Builds

---

# Performance Targets

Graph construction should scale approximately linearly with project size.

Graph queries should prioritize constant-time or logarithmic lookups where practical.

Memory usage should avoid duplicated nodes or edges.

---

# Testing

Every graph implementation requires

Correctness tests

Cycle detection tests

Large project benchmarks

Memory benchmarks

Serialization tests

Incremental update tests

Regression tests

---

# Future Extensions

Graph visualization

Architecture explorer

IDE navigation

Dependency heat maps

Security path explorer

Historical graph comparison

AI-assisted graph reasoning

---

# Success Criteria

The Graph Engine succeeds when

every engineering relationship is represented once,

rules query graphs rather than rebuilding them,

graph updates remain incremental,

and new graph types can be added without affecting existing APIs.

---

# Conclusion

The Graph Engine provides the relational foundation of MoveShield.

By centralizing graph construction and exposing stable query interfaces, it enables scalable, reusable, and performant engineering analysis across the entire platform.