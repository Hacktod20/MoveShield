# RFC-0015

# Plugin SDK

Status:
Draft

Version:
0.1.0

Target:
MoveShield Platform

---

# Abstract

This RFC defines the public Plugin SDK for MoveShield.

The SDK enables third-party developers to extend MoveShield without modifying the core engine.

Plugins are isolated components that contribute functionality through stable interfaces.

The core platform always owns execution.

Plugins only contribute behavior.

---

# Motivation

MoveShield should remain a platform.

Not every feature belongs in the main repository.

Organizations may want

Internal security rules

Architecture rules

Compliance policies

Custom reports

IDE integrations

Cloud integrations

Future AI assistants

Instead of modifying the engine,

these become plugins.

---

# Design Goals

The Plugin SDK must be

Stable

Versioned

Secure

Minimal

Well documented

Backward compatible

Deterministic

---

# Non Goals

Plugins must not

Replace the compiler

Modify MoveIR

Modify Findings

Modify the Knowledge Store

Bypass the Rule Engine

Access private engine state

The engine remains authoritative.

---

# Architecture

```

Core Engine

↓

Plugin Registry

↓

Plugin Manager

↓

Plugin Interface

↓

Plugin

```

---

# Plugin Types

Version One

Rule Plugin

Reporter Plugin

Formatter Plugin

CLI Plugin

Knowledge Plugin

Future

Cloud Plugin

Enterprise Plugin

Visualization Plugin

Language Server Plugin

AI Plugin

Telemetry Plugin

---

# Rule Plugin

Provides

New Rules

Rule Metadata

Documentation

Knowledge References

Rules execute exactly like built-in rules.

No special privileges exist.

---

# Reporter Plugin

Generates new output formats.

Examples

CSV

PDF

JUnit XML

Slack

Discord

Custom dashboards

The Reporting Engine invokes reporters through a stable interface.

---

# Formatter Plugin

Future support.

Examples

Architecture diagrams

Dependency graphs

GraphViz

Mermaid

Interactive visualizations

---

# Knowledge Plugin

Contributes engineering knowledge.

Examples

Organization metadata

Security advisories

Known vulnerability databases

Framework metadata

Knowledge plugins never modify compiler output.

---

# CLI Plugin

Adds commands.

Example

```

moveshield explain

moveshield metrics

moveshield audit

moveshield visualize

```

Commands execute through the CLI framework.

---

# Plugin Manifest

Every plugin contains

Identifier

Name

Version

Author

Description

License

Engine Compatibility

API Version

Capabilities

Dependencies

Digital Signature (future)

---

# Lifecycle

Discovery

↓

Validation

↓

Registration

↓

Initialization

↓

Execution

↓

Shutdown

Plugins are initialized once per analysis session.

---

# Plugin Registry

The registry stores

Identifier

Capabilities

Version

Status

Compatibility

Dependencies

Registration Metadata

The registry performs no execution.

---

# Plugin Manager

Responsibilities

Load plugins

Validate compatibility

Resolve dependencies

Initialize plugins

Unload plugins

Report failures

The manager isolates plugin failures.

---

# Version Compatibility

Every plugin declares

Minimum Engine Version

Maximum Engine Version

SDK Version

Rule Schema Version

Configuration Schema Version

Incompatible plugins are rejected before execution.

---

# Security

Plugins execute with limited privileges.

Plugins cannot

Modify engine memory

Replace built-in rules

Access internal caches

Override findings

Future enterprise versions may introduce

Sandboxing

Permission models

Plugin signing

Trust policies

---

# Error Isolation

Plugin failures must never terminate analysis.

The engine records

Plugin Identifier

Failure Reason

Stack Trace

Affected Features

Remaining plugins continue executing.

---

# Testing

Every plugin should provide

Unit Tests

Integration Tests

Compatibility Tests

Performance Tests

Regression Tests

Future certification tooling may validate plugins automatically.

---

# Distribution

Future distribution mechanisms

Official registry

Git repositories

Organization registries

Private enterprise registries

Signed packages

The SDK should not assume a single distribution model.

---

# Documentation

Every plugin must document

Purpose

Configuration

Capabilities

Examples

Compatibility

Limitations

Well documented plugins improve ecosystem adoption.

---

# Future Extensions

Remote plugins

Cloud execution

Marketplace

Automatic updates

Enterprise governance

Usage analytics

AI-generated plugins

None of these should require breaking the SDK.

---

# Success Criteria

The Plugin SDK succeeds when

developers can extend MoveShield without modifying the core engine,

plugins remain isolated,

compatibility is predictable,

and the engine evolves independently of third-party extensions.

---

# Conclusion

The Plugin SDK transforms MoveShield from a single application into a developer platform.

By defining stable extension points and preserving strict engine ownership, MoveShield can grow through an ecosystem of reusable plugins while maintaining long-term architectural stability.