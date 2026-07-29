# RFC-0014

# Cache System

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

The Cache System provides deterministic, version-aware caching for every expensive computation performed by MoveShield.

Caching is responsible for reducing repeated work between analyses while guaranteeing correctness.

No cached result may be reused unless it is proven to remain valid.

Correctness always takes priority over performance.

---

# Motivation

Static analysis consists of many expensive computations.

Examples include

Compilation

MoveIR construction

Graph generation

Complexity metrics

Dependency resolution

Capability analysis

Ownership analysis

Documentation analysis

Recomputing these for every execution wastes resources.

The Cache System enables MoveShield to reuse valid results while maintaining deterministic behavior.

---

# Design Goals

The Cache System must be

Correct

Deterministic

Incremental

Version Aware

Cross Platform

Thread Safe

Extensible

Transparent

---

# Non Goals

The Cache System does not

Generate findings

Execute rules

Compile projects

Read policies

Produce reports

Interpret engineering data

---

# Architecture

Filesystem

↓

Source Snapshot

↓

Hasher

↓

Cache Manager

↓

Cache Storage

↓

Analysis Engine

---

# Cache Levels

MoveShield defines four cache levels.

Level 1

Compiler Cache

Level 2

MoveIR Cache

Level 3

Graph Cache

Level 4

Knowledge Cache

Each level may be invalidated independently.

---

# Cache Keys

Every cache entry contains

Project Identifier

Compiler Version

Move Edition

MoveShield Version

Rule Pack Version

Configuration Hash

Source Hash

Dependency Hash

Platform Identifier

Only identical environments may reuse cached results.

---

# Source Hashing

Every source file is hashed.

Recommended algorithms should prioritize speed while maintaining extremely low collision probability.

The project hash is derived from

All source hashes

Manifest

Lockfile

Configuration

Compiler version

This guarantees deterministic invalidation.

---

# Cache Invalidation

Cache entries become invalid when

Source changes

Dependencies change

Compiler changes

Configuration changes

Rule pack changes

MoveShield version changes

Schema version changes

Invalidation must be automatic.

---

# Cache Storage

Version One

Local filesystem

Future

Shared cache

Remote cache

Cloud cache

Enterprise cache

Distributed cache

---

# Cache Directory

Recommended layout

.cache/

compiler/

moveir/

graphs/

knowledge/

reports/

metadata/

future/

Each cache domain remains isolated.

---

# Incremental Analysis

If one module changes

Only that module

Affected graphs

Affected knowledge

Affected rules

Need recomputation.

Everything else remains cached.

---

# Thread Safety

Multiple analysis workers may read cache concurrently.

Writes must be atomic.

Partially written cache entries are never visible.

---

# Serialization

Cache objects must support

Binary

JSON (debug)

Schema Version

Checksums

Compression (future)

---

# Performance

Goals

Minimal disk reads

Streaming writes

Memory mapping where beneficial

Zero-copy loading where practical

Avoid duplicate serialization

---

# Diagnostics

The Cache System records

Hit rate

Miss rate

Invalidation reason

Cache size

Average load time

Average write time

These metrics assist future optimization.

---

# Recovery

Corrupted cache entries

Must be discarded automatically.

Analysis continues using fresh computation.

Users should never manually repair cache.

---

# Security

Cache files contain engineering metadata.

Future versions may support

Signing

Encryption

Integrity verification

Remote trust validation

---

# Testing

Required tests

Correct invalidation

Version migration

Concurrent access

Corruption recovery

Performance

Large repositories

Regression

---

# Future Extensions

Remote build cache

CI cache sharing

Workspace cache federation

Cloud synchronization

Cross-device cache

Enterprise cache servers

---

# Success Criteria

The Cache System succeeds when

valid computations are never repeated,

invalid computations are never reused,

incremental analysis remains deterministic,

and cache management remains invisible to users.

---

# Conclusion

The Cache System provides the performance foundation for MoveShield.

By guaranteeing deterministic reuse of engineering computations, it enables large repositories to be analyzed efficiently while preserving correctness and reproducibility.