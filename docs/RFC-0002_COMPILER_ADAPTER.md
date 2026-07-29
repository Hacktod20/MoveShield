# RFC-0002

# Compiler Adapter Architecture

Status:
Draft

Author:
Muiz

Target Version:
v0.1.0

---

# Abstract

This RFC defines the architecture of the Compiler Adapter.

The Compiler Adapter is the boundary between the official Move compiler and MoveShield.

MoveShield shall never directly depend on compiler internals outside this layer.

The adapter exists to isolate compiler evolution from the rest of the application.

Every subsystem after compilation interacts exclusively with MoveShield's internal data model.

---

# Why This RFC Exists

The Move compiler is an actively developed project.

Future releases may introduce

• new APIs

• renamed structures

• changed metadata

• different diagnostics

If every subsystem communicates directly with compiler structures,

the entire project becomes tightly coupled to compiler changes.

Instead,

MoveShield introduces one stable boundary.

```

Official Compiler

↓

Compiler Adapter

↓

MoveShield Core

```

The compiler may change.

MoveShield should not.

---

# Responsibilities

The Compiler Adapter has only six responsibilities.

1.

Locate the project.

2.

Compile the package.

3.

Load compiler diagnostics.

4.

Load metadata.

5.

Normalize compiler output.

6.

Return MoveShield structures.

Nothing else.

---

# Non Responsibilities

The adapter must never

Run rules.

Generate reports.

Perform security analysis.

Compute complexity.

Generate findings.

Read configuration rules.

Those belong elsewhere.

---

# Design Goals

The adapter should be

Stable

Small

Deterministic

Replaceable

Testable

Version-aware

Minimal

---

# High Level Design

```

Move Project

↓

Move Compiler

↓

Compiler Output

↓

Compiler Adapter

↓

MoveShield Project

```

Everything below this line belongs to MoveShield.

Everything above belongs to the compiler.

---

# Inputs

The adapter receives

Project Path

Compiler Version

Configuration

Target Network

Build Options

Package Information

---

# Outputs

The adapter returns

Project

Packages

Modules

Functions

Structs

Enums (future)

Objects

Resources

Diagnostics

Source Locations

Attributes

Visibility

Dependencies

Compiler Metadata

Everything is converted into MoveShield objects.

---

# Why Normalize?

Suppose compiler version

2026.1

returns

```

FunctionVisibility

```

Later

2027.0

changes it to

```

VisibilityKind

```

Without normalization,

the entire MoveShield codebase breaks.

With normalization,

only the adapter changes.

Every other crate remains identical.

---

# Internal Pipeline

```

Locate Project

↓

Load Package

↓

Compile

↓

Collect Diagnostics

↓

Collect Metadata

↓

Normalize

↓

Return Project

```

Each stage has one responsibility.

---

# Adapter Interface

Every compiler implementation should expose the same interface.

Example

```rust
pub trait CompilerAdapter {

fn compile(
    &self,
    path: ProjectPath,
) -> Result<Project>;

}
```

Different compiler versions may have different implementations.

The interface never changes.

---

# Adapter Implementations

Future

```

Compiler Adapter

├── Move2026

├── Move2027

├── Experimental

```

This allows compiler upgrades without changing the analyzer.

---

# Project Object

The adapter always returns one object.

```

Project

```

Nothing else.

Project contains

Packages

Packages contain

Modules

Modules contain

Functions

Functions contain

Statements

Everything begins with

Project.

---

# Error Handling

Compilation failure

↓

CompilerError

Internal adapter failure

↓

AdapterError

Invalid project

↓

ProjectError

Configuration

↓

ConfigurationError

Errors remain separated.

---

# Logging

The adapter produces

Debug logs

Performance logs

Compiler diagnostics

It never produces

Security findings.

---

# Performance

Compilation should happen once.

Never compile twice during one scan.

All later analysis uses cached metadata.

---

# Version Awareness

Every project records

Compiler version

Language edition

Framework version

Target network

Future rules may depend upon version information.

---

# Future Support

The adapter should eventually support

Sui

Aptos

Private Move chains

Experimental compiler branches

This is achieved by implementing new adapters rather than rewriting existing ones.

---

# Testing

Unit Tests

Compilation Tests

Broken Projects

Large Projects

Performance Tests

Version Compatibility Tests

Regression Tests

---

# Success Criteria

The rest of MoveShield should never know

which compiler version produced the project.

Only the adapter knows.

This isolation guarantees long-term maintainability.

---

# Future RFCs

RFC-0003

Core Data Model

will define exactly what

Project

contains.

This RFC intentionally treats

Project

as an opaque object.

The next RFC defines it.

---

# Conclusion

The Compiler Adapter is the foundation of MoveShield's architecture.

It allows the compiler to evolve independently while keeping the remainder of the system stable.

Every future subsystem depends upon the stability provided by this abstraction.

Without the Compiler Adapter,

MoveShield would become tightly coupled to upstream compiler changes and significantly harder to maintain over time.