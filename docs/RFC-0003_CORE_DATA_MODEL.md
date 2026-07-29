# RFC-0003

# Core Data Model

Status:
Draft

Version:
0.1.0

Author:
Muiz

---

# Abstract

This RFC defines the Core Data Model (CDM).

The Core Data Model is the canonical representation of every Move project inside MoveShield.

Every subsystem communicates through the Core Data Model.

The compiler produces it.

The analyzer consumes it.

Rules inspect it.

Reports serialize it.

No subsystem should directly depend on compiler-specific structures.

---

# Motivation

Compilers evolve.

Internal APIs change.

Data structures change.

MoveShield must remain stable.

The Core Data Model isolates compiler evolution from MoveShield's engineering systems.

The CDM represents engineering concepts rather than compiler implementation details.

---

# Design Principles

The model must satisfy six requirements.

1.

Immutable

After construction the model is read-only.

---

2.

Serializable

Every object can become JSON.

---

3.

Versioned

Every structure supports schema evolution.

---

4.

Language Aware

Designed specifically for Move.

---

5.

Compiler Independent

The CDM should not expose compiler implementation details.

---

6.

Minimal

Only information required by engineering analysis should exist.

---

# High Level Structure

```

Project

│

├── Packages

│ ├── Modules

│ │ ├── Structs

│ │ ├── Enums (future)

│ │ ├── Functions

│ │ ├── Constants

│ │ ├── Events

│ │ └── Friends

│

├── Dependencies

├── Compiler

├── Diagnostics

├── Metadata

└── Statistics

```

Everything begins with

Project.

---

# Project

Represents one scanned repository.

Contains

Project Name

Version

Root Path

Compiler Version

Move Edition

Packages

Dependencies

Metadata

Statistics

Configuration

One scan always produces one Project.

---

# Package

Represents one Move package.

Contains

Package Name

Manifest

Modules

Dependencies

Named Addresses

Edition

Target Network

Compiler Settings

---

# Module

Represents one Move module.

Contains

Name

Address

Source File

Visibility

Functions

Structs

Events

Constants

Imports

Friends

Attributes

Metrics

---

# Function

Represents one Move function.

Contains

Identifier

Visibility

Parameters

Return Types

Attributes

Documentation

Location

Complexity

Call Graph

Security Metadata

Ownership Metadata

Resource Metadata

Version Metadata

---

# Struct

Represents one Move struct.

Contains

Name

Abilities

Fields

Attributes

Visibility

Documentation

Usage Graph

Metrics

---

# Resource

Represents every resource type.

Contains

Owner

Abilities

Transfer Rules

Construction

Destruction

Mutation Points

Sharing

Storage

---

# Capability

Represents authorization capabilities.

Contains

Capability Type

Owner

Creation

Transfer

Usage

Destruction

Visibility

Relationships

Capabilities become first-class citizens.

They are not inferred repeatedly.

---

# Event

Represents emitted events.

Contains

Fields

Visibility

Emit Locations

Documentation

Consumers

---

# Dependency

Represents every imported package.

Contains

Version

Address

Relationship

Kind

Usage

Security Score (future)

---

# Compiler Metadata

Compiler metadata remains separate.

Contains

Compiler Version

Edition

Optimization

Warnings

Feature Flags

Target

Platform

---

# Source Location

Every object records location.

File

Line

Column

Span

Length

This enables

IDE support

Reports

Quick Fixes

GitHub annotations

---

# Metrics

Every engineering object stores metrics.

Examples

Cyclomatic Complexity

Statement Count

Function Length

Dependency Count

Fan-In

Fan-Out

Documentation Coverage

Visibility Score

Metrics are descriptive.

Rules decide how to interpret them.

---

# Relationships

The model contains explicit relationships.

Example

Function

↓

Calls

↓

Function

Example

Struct

↓

Owns

↓

Resource

Example

Capability

↓

Protects

↓

Function

Relationships eliminate repeated graph construction.

---

# Graphs

The CDM stores reusable graphs.

Call Graph

Dependency Graph

Ownership Graph

Capability Graph

Import Graph

Visibility Graph

Future

Control Flow Graph

Data Flow Graph

Rules reuse graphs instead of rebuilding them.

---

# Metadata

Every object supports metadata.

Metadata may be added by

Compiler

Analyzer

Plugins

Future Engines

Metadata never changes the original object.

---

# Diagnostics

Compiler diagnostics remain unchanged.

MoveShield diagnostics remain separate.

Compiler

↓

Syntax Error

MoveShield

↓

Engineering Finding

Never mix them.

---

# Statistics

The Project stores aggregate information.

Example

Modules

Functions

Resources

Capabilities

Events

Average Complexity

Maximum Complexity

Dependencies

Rule Count

Analysis Duration

Statistics support reporting.

---

# Object Identity

Every object receives a stable identifier.

UUIDs are unnecessary.

Instead

Project

↓

Package

↓

Module

↓

Function

can construct deterministic IDs.

Example

```

0x2::coin::transfer

```

Stable IDs improve caching.

---

# Serialization

Every object must support

JSON

Binary (future)

Caching

Hashing

Diffing

This enables incremental analysis.

---

# Ownership Rules

Objects become immutable after construction.

Analysis never mutates objects.

Rules generate Findings.

Never modifications.

---

# Memory Model

One Project exists in memory.

Rules receive shared references.

No copies.

No cloning.

Read-only.

This minimizes memory usage.

---

# Future Extensions

The CDM should eventually support

AI Metadata

Gas Metadata

Execution Profiles

Historical Analysis

Diff Analysis

Version Compatibility

Cross Repository Analysis

Enterprise Policies

without breaking existing APIs.

---

# Success Criteria

The Core Data Model succeeds when

Every subsystem depends only upon it.

Compiler upgrades require only adapter updates.

Rules never inspect compiler internals.

Plugins remain stable.

Reports remain independent.

Future integrations reuse the same objects.

---

# Conclusion

The Core Data Model is the heart of MoveShield.

It is the common language spoken by every subsystem.

A stable, expressive, and compiler-independent model enables MoveShield to evolve without constant architectural rewrites.

Every engineering decision made after this RFC assumes the existence of the Core Data Model.



                    Official Move Compiler
                             │
                             ▼
                   Compiler Adapter
                             │
                             ▼
                 Core Data Model (OUR IP)
                             │
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
   Static Analyzer      Linter Engine      Formatter
        ▼                    ▼                    ▼
   Security Rules      Style Rules      Code Actions
        └────────────────────┼────────────────────┘
                             ▼
                      Language Server
                             ▼
        VS Code • Zed • JetBrains • GitHub • CLI