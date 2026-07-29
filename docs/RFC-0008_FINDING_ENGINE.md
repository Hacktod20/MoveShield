# RFC-0008

# Finding Engine

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

The Finding Engine transforms engineering observations into standardized findings.

The Rule Engine identifies engineering observations.

The Finding Engine interprets those observations using configured policies and produces deterministic findings suitable for developers, CI systems, IDEs, reports, and future cloud services.

The Finding Engine is the final decision layer before reporting.

---

# Motivation

Rules should never decide

whether something is

Error

Warning

Advice

Information

or

Ignore.

Those are policy decisions.

Instead,

rules produce observations.

The Finding Engine evaluates those observations consistently.

---

# Design Philosophy

Facts should remain independent from policy.

Engineering facts rarely change.

Organizational policy changes frequently.

Separating the two dramatically improves flexibility.

---

# Pipeline

```

Compiler

↓

Analysis Engine

↓

Knowledge Store

↓

Rules

↓

Observations

↓

Finding Engine

↓

Findings

↓

Reports

```

---

# Observation

An Observation represents a technical fact discovered by analysis.

Example

```

Observation

Rule:

MS001

Function:

transfer()

Complexity:

17

Location:

module.move:42

```

This observation contains no judgment.

---

# Finding

A Finding represents a decision.

```

Finding

Rule:

MS001

Severity:

Warning

Confidence:

High

Location:

module.move:42

Recommendation:

Reduce function complexity.

```

---

# Why Separate Them?

Imagine two companies.

Company A

Maximum complexity

10

Company B

Maximum complexity

20

Observation

Complexity = 15

Company A

↓

Warning

Company B

↓

No Finding

The observation stays identical.

Only policy changes.

---

# Finding Lifecycle

```

Observation

↓

Validation

↓

Policy Evaluation

↓

Severity Resolution

↓

Confidence Assignment

↓

Deduplication

↓

Finding

```

---

# Validation

Every observation is validated.

Required fields

Rule ID

Location

Evidence

Knowledge References

Category

Missing information causes rejection.

---

# Policy Evaluation

Policies decide

Enabled

Disabled

Ignored

Organization Override

Experimental

Suppressed

Policies never modify observations.

---

# Severity Resolution

Supported severities

Critical

High

Medium

Low

Informational

Hidden

Organizations may override defaults.

---

# Confidence

Confidence measures certainty.

High

Medium

Low

Experimental

Confidence should be based on evidence quality.

Not intuition.

---

# Categories

Every finding belongs to one category.

Security

Performance

Architecture

Maintainability

Documentation

Compatibility

Upgrade Safety

Gas

Style

Future categories may be added without breaking compatibility.

---

# Evidence

Every finding must include evidence.

Example

Affected Function

Affected Module

Affected Capability

Relevant Graph

Relevant Metric

Supporting Knowledge

This allows developers to understand why the finding exists.

---

# Suggested Fix

Every finding should include guidance whenever possible.

Levels

Automatic Fix

Suggested Refactor

Documentation Link

Manual Investigation

Future AI Explanation

Rules should never emit vague messages.

---

# References

Findings may include references to

Official Move documentation

Sui documentation

MoveShield Knowledge Base

Security advisories

Audit reports

Academic research

References increase trust.

---

# Suppression

Developers may suppress findings.

Methods

Configuration

Inline annotation (future)

Project policy

Organization policy

Suppressed findings remain available internally for reporting if configured.

---

# Deduplication

Multiple rules may detect the same engineering issue.

The Finding Engine merges equivalent findings where appropriate.

This prevents overwhelming developers with duplicate messages.

---

# Stable Identifier

Every finding receives a deterministic identifier.

Components

Rule ID

File

Span

Symbol

Project

Stable identifiers support

Historical tracking

IDE updates

Incremental analysis

Cloud synchronization

---

# Output Contract

Every finding contains

Identifier

Rule ID

Category

Severity

Confidence

Location

Primary Message

Detailed Explanation

Evidence

Suggested Fix

References

Related Symbols

Timestamp

Engine Version

Schema Version

---

# Serialization

Supported formats

JSON

SARIF

Binary (future)

Cloud API (future)

The internal schema remains format-independent.

---

# Performance

The Finding Engine must process thousands of observations efficiently.

Requirements

Streaming support

Minimal allocations

Parallel aggregation

Deterministic ordering

---

# Testing

Required tests

Schema validation

Deduplication

Policy overrides

Severity overrides

Serialization

Regression

Performance

---

# Future Extensions

Organization Policies

Compliance Profiles

Industry Standards

Historical Trends

Machine Learning Ranking

AI Explanation

Risk Prediction

None of these should require changes to the observation model.

---

# Success Criteria

The Finding Engine succeeds when

every observation becomes a consistent finding,

policy changes require no rule changes,

reports remain deterministic,

and developers trust the output.

---

# Conclusion

The Finding Engine is the policy layer of MoveShield.

It separates engineering knowledge from engineering judgment.

By introducing a deterministic transformation from observations to findings, MoveShield gains flexibility, maintainability, enterprise readiness, and long-term architectural stability.