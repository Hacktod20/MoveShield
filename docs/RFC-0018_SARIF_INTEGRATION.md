# RFC-0018

# SARIF Integration

Status:
Draft

Version:
0.1.0

Target:
MoveShield Reporting

---

# Abstract

This RFC defines MoveShield's implementation of the Static Analysis Results Interchange Format (SARIF) 2.1.0.

SARIF is the primary machine-readable output format for MoveShield.

It enables integration with GitHub Code Scanning, security dashboards, CI systems, and enterprise tooling without requiring custom adapters.

SARIF generation is the responsibility of the Report Engine.

---

# Motivation

Developers already use platforms that understand SARIF.

Rather than inventing a proprietary format,

MoveShield adopts the industry standard.

Benefits include

Native GitHub support

Portable scan results

Interoperability

Long-term compatibility

Standardized tooling

---

# Design Goals

The SARIF implementation must be

Fully compliant

Deterministic

Versioned

Machine readable

Streaming capable

Extensible

Backward compatible

---

# Non Goals

SARIF generation does not

Run analysis

Assign severities

Modify findings

Execute rules

Interpret policies

It only serializes Findings.

---

# Architecture

Analysis Engine

↓

Findings

↓

Report Engine

↓

SARIF Builder

↓

SARIF Document

↓

File

---

# Supported Version

MoveShield targets

SARIF Version 2.1.0

Future versions may support newer revisions while maintaining backward compatibility.

---

# Mapping

Every Finding maps to one SARIF Result.

Every Rule maps to one SARIF Rule.

Every Location maps to one Physical Location.

Every Recommendation maps to a Help section.

---

# Rule Metadata

Each exported rule includes

Identifier

Name

Category

Description

Help Text

Default Severity

References

Documentation URL

Tags

---

# Result Metadata

Each result contains

Rule ID

Severity

Message

Primary Location

Related Locations

Evidence

Fix Recommendation

Confidence

Category

---

# Locations

Every finding must include

File

Start Line

Start Column

End Line

End Column

Symbol

Module

Package

Location precision improves developer experience.

---

# Severity Mapping

MoveShield

Critical

High

Medium

Low

Informational

Mapped into SARIF levels using a deterministic translation strategy.

The original MoveShield severity is preserved in custom properties.

---

# Rule Help

Every exported rule should contain

Description

Engineering rationale

Secure coding guidance

Examples

Reference links

This allows IDEs and GitHub to present useful remediation guidance.

---

# Related Locations

When available

Additional source locations should be attached.

Examples

Call sites

Dependency locations

Capability definitions

Ownership sources

This improves investigation.

---

# Automation Details

Every SARIF report includes

Engine Name

Engine Version

Rule Pack Version

Policy Version

Configuration

Analysis Timestamp

---

# Custom Properties

MoveShield-specific metadata may include

Confidence

Knowledge references

Capability identifiers

Complexity metrics

Project metadata

These are stored using SARIF extension properties.

---

# Output

Default filename

moveshield.sarif

Users may specify

--output custom.sarif

---

# Validation

Every generated SARIF file should pass schema validation before release.

Regression tests include

Schema compliance

GitHub upload validation

Large project output

Encoding

Deterministic ordering

---

# Performance

The SARIF writer should

Support streaming

Avoid duplicate allocations

Reuse serialized metadata

Handle very large repositories efficiently

---

# CLI

Examples

moveshield analyze --format sarif

moveshield report --format sarif

moveshield report --output reports/security.sarif

---

# Future Extensions

Automatic GitHub upload

SARIF compression

Incremental SARIF

Enterprise metadata

Historical comparison

Cloud synchronization

---

# Success Criteria

The SARIF integration succeeds when

generated reports are accepted by GitHub,

every finding is faithfully represented,

reports remain deterministic,

and new metadata can be added without breaking compatibility.

---

# Conclusion

SARIF is the standard interchange format for MoveShield.

By adopting SARIF 2.1.0 as the primary machine-readable output, MoveShield integrates naturally with modern development workflows while remaining independent of any single hosting platform.