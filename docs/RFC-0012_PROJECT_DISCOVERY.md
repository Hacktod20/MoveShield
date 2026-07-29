# RFC-0012

# Project Discovery

Status:
Draft

Version:
0.1.0

---

# Abstract

The Project Discovery subsystem locates, validates, and prepares Move projects for analysis.

It is the first executable stage of the MoveShield pipeline.

Its responsibility is identifying projects—not compiling them.

---

# Motivation

Before analysis begins,

MoveShield must answer

Where is the project?

What packages belong to it?

Which edition is used?

Which dependencies exist?

Which compiler configuration applies?

Rather than distributing this logic across multiple components,

Project Discovery centralizes it.

---

# Responsibilities

Locate Move.toml

Load Move.lock

Identify workspace root

Resolve local dependencies

Resolve git dependencies

Read package metadata

Collect build configuration

Prepare compiler inputs

---

# Non Responsibilities

Compilation

Parsing

Static analysis

Reporting

Rule execution

Graph construction

---

# Discovery Pipeline

Filesystem

↓

Locate Move.toml

↓

Load Manifest

↓

Resolve Dependencies

↓

Validate Structure

↓

Workspace Model

↓

Compiler Adapter

---

# Entry Point

Analysis begins from

ProjectRoot

Every scan targets one root directory.

The root contains

Move.toml

or

a supported workspace layout.

---

# Manifest

The manifest is the source of truth.

Project Discovery extracts

Package Name

Version

Edition

Addresses

Dependencies

Build Profiles

Named Addresses

Compiler Options

---

# Workspace Model

MoveShield internally constructs

Workspace

↓

Packages

↓

Modules

↓

Sources

↓

Tests

↓

Scripts

This model is immutable.

---

# Dependency Resolution

Supported dependency types

Local

Git

Framework

Future Registry

Each dependency records

Source

Revision

Path

Resolved Location

Status

---

# Validation

Before compilation

Project Discovery validates

Manifest syntax

Directory layout

Dependency integrity

Duplicate modules

Missing source files

Unsupported editions

Invalid addresses

Failures stop the pipeline early.

---

# Output

Project Discovery returns

Workspace

Package Metadata

Compiler Configuration

Dependency Graph

Filesystem Metadata

Nothing compiler-specific exists yet.

---

# Performance

Requirements

Single filesystem traversal

Parallel dependency loading where possible

Manifest caching

Deterministic ordering

Minimal allocations

---

# Future

Multi-package repositories

Remote package cache

Workspace watch mode

Incremental discovery

IDE integration

---

# Success Criteria

Project Discovery succeeds when

every valid Move project is normalized into one immutable Workspace model before entering the Compiler Adapter.
