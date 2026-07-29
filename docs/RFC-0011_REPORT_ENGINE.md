# RFC-0011

# Report Engine

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

The Report Engine transforms Findings into developer-facing outputs.

It is the only subsystem responsible for presentation.

Every previous subsystem exists to discover engineering knowledge.

The Report Engine exists to communicate that knowledge.

Reports must remain independent of analysis.

Changing report formats must never affect the analysis engine.

---

# Motivation

Many static analysis tools tightly couple

analysis

and

presentation.

This makes maintenance difficult.

Instead,

MoveShield follows a layered architecture.

Analysis ends with Findings.

Reporting begins with Findings.

Nothing before the Report Engine knows

HTML,

Markdown,

JSON,

or SARIF.

---

# Design Principles

Reports must be

Deterministic

Streaming

Format Independent

Versioned

Composable

Accessible

Machine Readable

Human Friendly

---

# Responsibilities

The Report Engine

Formats findings.

Groups findings.

Orders findings.

Calculates summaries.

Exports multiple formats.

Generates developer guidance.

Produces CI outputs.

Produces IDE outputs.

Nothing more.

---

# Non Responsibilities

The Report Engine never

Runs rules.

Compiles projects.

Reads source code.

Builds graphs.

Computes complexity.

Assigns severity.

Changes findings.

---

# Pipeline

Compiler

↓

Analysis

↓

Knowledge

↓

Observations

↓

Policy

↓

Findings

↓

Report Engine

↓

Output

---

# Supported Outputs

Version One

Terminal

JSON

Markdown

SARIF

HTML

Future

PDF

Cloud API

Interactive Dashboard

VS Code Diagnostics

JetBrains Plugin

GitHub PR Review

---

# Terminal Report

Purpose

Fast local feedback.

Characteristics

Colorized.

Grouped by severity.

Summary first.

Detailed findings second.

Exit codes suitable for CI.

---

# JSON

Purpose

Automation.

Characteristics

Stable schema.

Machine readable.

Versioned.

Supports future APIs.

---

# Markdown

Purpose

Documentation.

Characteristics

Human readable.

Suitable for GitHub comments.

Suitable for audit reports.

---

# HTML

Purpose

Interactive reports.

Characteristics

Searchable.

Collapsible sections.

Project summary.

Charts.

Architecture overview.

Future graph visualizations.

---

# SARIF

Purpose

GitHub Code Scanning.

MoveShield should emit SARIF 2.1.0 compatible output so findings can appear directly inside pull requests and repository security views. :contentReference[oaicite:2]{index=2}

---

# Report Structure

Every report contains

Project Information

Scan Metadata

Summary

Statistics

Findings

Rule References

Knowledge References

Engine Version

Policy Version

Generation Timestamp

---

# Executive Summary

Every report begins with

Overall Risk

Critical Findings

High Findings

Medium Findings

Low Findings

Informational Findings

Scan Duration

Project Statistics

---

# Finding Section

Each finding contains

Rule Identifier

Category

Severity

Confidence

Location

Description

Evidence

Recommendation

References

Knowledge Links

Related Symbols

---

# Statistics

Reports include

Files

Packages

Modules

Functions

Structs

Resources

Capabilities

Average Complexity

Documentation Coverage

Dependency Count

Public APIs

Analysis Time

---

# Exit Codes

CLI exit codes

0

No blocking findings.

1

Analysis failure.

2

Blocking findings.

Future codes may distinguish

Configuration errors,

Plugin failures,

Internal engine failures.

---

# Versioning

Every report embeds

Schema Version

Engine Version

Policy Version

Rule Pack Version

This guarantees reproducibility.

---

# Accessibility

Reports should

Support screen readers.

Avoid color-only indicators.

Provide machine-readable metadata.

Support plain-text environments.

---

# Future Features

Interactive architecture diagrams.

Call graph visualization.

Ownership graph visualization.

Capability explorer.

Historical comparison.

Trend analysis.

AI-generated explanations.

Enterprise dashboards.

---

# Success Criteria

The Report Engine succeeds when

developers can understand findings immediately,

machines can consume structured output,

CI systems integrate seamlessly,

and new report formats require no changes to the analysis engine.

---

# Conclusion

The Report Engine is the presentation layer of MoveShield.

Its purpose is to communicate engineering intelligence clearly and consistently while remaining completely independent from analysis logic.

This separation ensures that future integrations—GitHub, IDEs, cloud dashboards, and APIs—can be added without changing the core engine.