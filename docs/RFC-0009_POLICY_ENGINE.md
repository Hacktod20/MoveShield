# RFC-0009

# Policy Engine

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

The Policy Engine transforms engineering observations into organization-specific decisions.

Rules discover facts.

Policies decide what those facts mean.

This separation allows MoveShield to remain flexible across open-source projects, startups, enterprises, educational institutions, and security audit firms.

The Policy Engine never discovers engineering problems.

It evaluates engineering knowledge against configurable organizational standards.

---

# Motivation

Imagine three projects.

Project A

A personal learning project.

Project B

A DeFi protocol securing hundreds of millions of dollars.

Project C

An enterprise financial application.

All three produce the same observation.

```

Function Complexity = 14

```

Should the result be identical?

No.

Each organization has different engineering expectations.

Therefore,

engineering observations must remain separate from organizational policy.

---

# Design Principles

The Policy Engine must satisfy the following principles.

• Policies never inspect compiler internals.

• Policies never execute static analysis.

• Policies consume observations only.

• Policies remain deterministic.

• Policies are versioned.

• Policies are portable.

• Policies are testable.

• Policies are composable.

---

# High-Level Architecture

```

Compiler

↓

Analysis Engine

↓

Knowledge Store

↓

Rule Engine

↓

Observations

↓

Policy Engine

↓

Finding Engine

↓

Reports

```

---

# Observation

Example

```

Observation

Rule:
MS-SEC-001

Symbol:
transfer()

Metric:
Capability Validation

Evidence:
Missing

```

No severity exists.

No warning exists.

No recommendation exists.

Only facts.

---

# Policy

A policy answers one question.

"What should this organization do with this observation?"

Possible actions

Ignore

Inform

Warn

Error

Critical

Require Manual Review

Future

Require Approval

Block Merge

Require Audit

Escalate

---

# Policy Types

MoveShield supports multiple policy profiles.

Default

Strict

Enterprise

Audit

Educational

Experimental

Custom

---

# Default Policy

Designed for individual developers.

Characteristics

Minimal configuration.

Balanced severity.

Helpful educational guidance.

Suitable for most projects.

---

# Strict Policy

Designed for production blockchain applications.

Characteristics

Lower tolerance.

More warnings.

Stronger architectural requirements.

Recommended before mainnet deployment.

---

# Enterprise Policy

Designed for organizations.

Supports

Approval rules.

Organization overrides.

Compliance requirements.

Historical tracking.

Signed policy bundles.

---

# Educational Policy

Purpose

Teach engineering.

Many observations become educational guidance rather than warnings.

Output includes

Examples.

References.

Suggested reading.

Knowledge Base links.

---

# Audit Policy

Purpose

Assist professional security reviews.

Characteristics

Minimal suppression.

Highest verbosity.

Maximum evidence.

Architecture metadata.

Graph visualization.

Rule traceability.

---

# Policy Structure

Each policy contains

Identifier

Version

Description

Enabled Rules

Severity Overrides

Confidence Overrides

Ignored Paths

Ignored Modules

Ignored Symbols

Knowledge References

Metadata

---

# Rule Overrides

Example

```

MS-ARCH-004

Default

Medium

Enterprise

High

Audit

Critical

Educational

Informational

```

The rule remains unchanged.

Only policy changes.

---

# Severity Resolution

The Policy Engine maps observations into findings.

Supported severities

Critical

High

Medium

Low

Informational

Suppressed

Organizations may introduce custom labels while preserving internal canonical levels.

---

# Confidence Adjustment

Policies may adjust confidence thresholds.

Example

Experimental rules

↓

Low Confidence

↓

Hidden

Production rules

↓

High Confidence

↓

Visible

---

# Suppression

Supported suppression mechanisms

Project configuration

Policy file

Future inline suppression

Temporary suppression

Organization suppression

Every suppression must include metadata explaining why it exists.

---

# Organization Policies

Future enterprise deployments may define

Engineering standards.

Architecture standards.

Security standards.

Naming conventions.

Documentation requirements.

Upgrade requirements.

The Policy Engine enforces these consistently across repositories.

---

# Policy Composition

Policies may inherit from one another.

Example

```

Default

↓

Strict

↓

Enterprise

↓

Company Policy

↓

Project Policy

```

Overrides are applied from general to specific.

---

# Policy Validation

Before execution,

every policy is validated.

Validation checks

Schema version.

Duplicate rules.

Unknown rules.

Invalid severities.

Circular inheritance.

Missing references.

Invalid overrides.

---

# Performance

Policy evaluation should remain inexpensive.

Target characteristics

O(number of observations)

Streaming evaluation.

Minimal allocations.

Deterministic ordering.

Policies should never become a performance bottleneck.

---

# Versioning

Every policy contains

Schema Version

Policy Version

Compatible Engine Version

Compatible Rule Versions

Future versions may include migration tooling.

---

# Testing

Required tests

Schema validation.

Override behavior.

Inheritance.

Suppression.

Serialization.

Regression.

Performance.

---

# Future Extensions

Future capabilities include

Compliance frameworks.

Enterprise governance.

Approval workflows.

Historical policy evolution.

Remote policy distribution.

Signed policy packages.

Policy marketplace.

Machine-generated recommendations.

---

# Success Criteria

The Policy Engine succeeds when

the same engineering observations can produce different outcomes depending on organizational requirements,

without requiring any modification to analysis rules.

---

# Conclusion

The Policy Engine separates engineering knowledge from organizational expectations.

This separation enables MoveShield to support individual developers, startups, enterprises, educators, and audit firms through a single architecture while maintaining deterministic behavior and long-term maintainability.