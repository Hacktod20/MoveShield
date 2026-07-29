# RFC-0005

# Rule Engine

Status:
Draft

Version:
0.1.0

Target:
MoveShield Core

---

# Abstract

The Rule Engine is responsible for executing engineering rules against a compiled Move project.

A rule is an independent unit of analysis.

Rules inspect the Core Data Model.

Rules never modify it.

Rules never communicate with one another.

Rules never generate reports.

Rules only generate Findings.

The Rule Engine coordinates rule discovery, execution, lifecycle management, versioning, performance, and compatibility.

---

# Motivation

Without a Rule Engine,

every new analysis feature would require modifications throughout the codebase.

That creates

tight coupling,

poor maintainability,

and slow feature development.

Instead,

MoveShield treats every analysis as an independent rule.

Examples include

Missing capability checks

Unused resources

Excessive function complexity

Public API misuse

Dead code

Naming conventions

Documentation coverage

Future gas optimization

Upgrade compatibility

Every one of these is simply another rule.

---

# Goals

The Rule Engine must satisfy the following goals.

• Rules are independent.

• Rules are deterministic.

• Rules are testable.

• Rules are versioned.

• Rules are configurable.

• Rules can execute in parallel.

• Rules can be added without modifying existing rules.

---

# Non Goals

The Rule Engine does not

Compile projects.

Generate reports.

Render HTML.

Upload data.

Communicate with GitHub.

Store cloud state.

Manage configuration files.

Those belong to other components.

---

# High-Level Architecture

```

Project

↓

Analysis Context

↓

Rule Registry

↓

Rule Scheduler

↓

Rule Executor

↓

Finding Aggregator

↓

Findings

```

---

# Analysis Context

Every rule receives the same immutable context.

```
AnalysisContext

Project

Graphs

Metrics

Metadata

Configuration

Knowledge Base

Compiler Version

Language Edition

Workspace Information
```

Rules never receive compiler internals.

---

# Rule Interface

Every rule implements a common interface.

```rust
pub trait Rule {
    fn id(&self) -> RuleId;

    fn name(&self) -> &'static str;

    fn category(&self) -> RuleCategory;

    fn severity(&self) -> Severity;

    fn analyze(
        &self,
        context: &AnalysisContext,
    ) -> Vec<Finding>;
}
```

This interface should remain stable across major versions.

---

# Rule Metadata

Every rule declares metadata.

Identifier

Human Name

Category

Description

Default Severity

Confidence

Version

Documentation Link

References

Tags

Examples

Estimated Runtime

Auto Fix Support

Experimental Status

This metadata powers

CLI help

IDE tooltips

Documentation

Configuration

Filtering

Reporting

---

# Rule Categories

Version 1

Security

Architecture

Performance

Maintainability

Documentation

Complexity

Style

Compatibility

Future

Gas

Upgrade Safety

Object Lifecycle

Dependency Quality

AI Suggestions

Accessibility

---

# Rule Lifecycle

```

Discover

↓

Register

↓

Validate

↓

Schedule

↓

Execute

↓

Generate Findings

↓

Dispose

```

Rules remain stateless.

---

# Rule Discovery

Rules may originate from

Built-in rules

Plugin rules

Enterprise rules

Experimental rules

The engine loads them into a registry.

---

# Rule Registry

The registry stores

Identifier

Version

Category

Enabled status

Compatibility

Dependencies

Runtime metadata

The registry never executes rules.

---

# Rule Scheduler

Responsibilities

Determine execution order.

Resolve dependencies.

Group parallel-safe rules.

Allocate worker threads.

Prevent duplicate execution.

The scheduler optimizes performance.

---

# Rule Execution

Every rule executes against

the same immutable context.

Rules may not

modify context,

modify findings from other rules,

or depend on execution order.

This ensures reproducible results.

---

# Finding Contract

Rules return Findings.

Never logs.

Never strings.

Never HTML.

Every finding includes

Rule ID

Severity

Confidence

Location

Explanation

Suggested Fix

Related Symbols

Knowledge Links

References

---

# Rule Configuration

Users may

Disable rules.

Override severity.

Ignore files.

Ignore modules.

Enable experimental rules.

Apply organization policies.

Rules never read configuration directly.

Configuration is resolved before execution.

---

# Versioning

Every rule has an independent version.

Major

Breaking logic changes.

Minor

Improved detection.

Patch

Bug fixes.

This enables tracking changes in rule behavior over time.

---

# Parallel Execution

Rules should execute concurrently whenever possible.

Requirements

No shared mutable state.

Immutable analysis context.

Independent outputs.

Deterministic aggregation.

Parallelism must never change findings.

---

# Error Isolation

If one rule fails,

the engine records the failure,

reports it,

and continues executing remaining rules.

One faulty rule must never terminate analysis.

---

# Performance Budget

Every rule declares an estimated runtime class.

Small

Medium

Large

The scheduler uses this information to balance workloads.

Future versions may enforce execution time limits.

---

# Testing

Every rule requires

Unit tests

Regression tests

Positive cases

Negative cases

False positive tests

False negative tests

Performance benchmarks

Rules failing required tests cannot be merged.

---

# Knowledge Integration

Every rule links to the MoveShield Knowledge Base.

A finding should answer

What happened?

Why is this important?

How do I fix it?

Where can I learn more?

This turns MoveShield into both a scanner and an educational platform.

---

# Future Extensions

Future versions may support

Rule dependencies

Organization-specific policies

Marketplace rules

Signed rule packages

Remote rule updates

AI-assisted rule generation

Cross-project analysis

Historical trend analysis

The core interface should remain stable.

---

# Success Criteria

The Rule Engine succeeds when

new rules require no changes to existing code,

execution remains deterministic,

parallel execution scales,

developers can understand findings,

and contributors can extend the platform confidently.

---

# Conclusion

The Rule Engine is the extensibility layer of MoveShield.

By treating every analysis as an independent, versioned, deterministic rule, MoveShield gains the flexibility to grow from a security scanner into a comprehensive engineering platform without sacrificing maintainability or developer trust.