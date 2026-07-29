# MoveShield

# 0002 — Competitor Analysis

Version: 0.1.0

Status:
Draft

Author:
Muiz

Last Updated:
July 2026

---

# Purpose

This document answers one question.

Why should MoveShield exist if other tools already exist?

Every successful engineering product competes against existing solutions.

Sometimes those solutions are software.

Sometimes they are manual workflows.

Sometimes they are habits.

MoveShield competes against all three.

This document exists to understand that landscape.

---

# Engineering Principle

We do not compete by copying.

We compete by solving problems better.

Understanding competitors is not about imitation.

It is about discovering gaps.

---

# Market Overview

Current Move tooling can be divided into six categories.

Category 1

Compiler

Category 2

Formal Verification

Category 3

Security Audits

Category 4

Static Analysis

Category 5

Developer Tooling

Category 6

Manual Review

Every existing solution solves only part of the software quality lifecycle.

MoveShield aims to connect these pieces into one workflow.

---

# Competitor 1

Official Move Compiler

Purpose

Compile Move source code.

Responsibilities

• Parsing

• Type checking

• Bytecode generation

• Compiler diagnostics

Strengths

• Official

• Fast

• Stable

• Trusted

Weaknesses

The compiler only answers one question.

"Can this code compile correctly?"

It does not answer

"Is this architecture safe?"

It does not answer

"Will this permission model fail?"

It does not answer

"Is this implementation maintainable?"

Conclusion

The compiler is a foundation.

Not a quality platform.

MoveShield builds on top of it.

---

# Competitor 2

Move Bytecode Verifier

Purpose

Verify safety properties of compiled bytecode.

Responsibilities

• Type safety

• Reference safety

• Resource correctness

Strengths

Extremely important.

Prevents invalid bytecode.

Weaknesses

Focused on language correctness.

Not engineering quality.

Conclusion

MoveShield should never duplicate verifier logic.

Instead,

MoveShield should consume verified bytecode.

---

# Competitor 3

Move Prover

Purpose

Formal verification.

Strengths

Mathematical guarantees.

Powerful.

Research-backed.

Weaknesses

Requires specifications.

Steep learning curve.

Difficult for beginners.

Many teams never use it consistently.

MoveShield Position

We complement the prover.

MoveShield helps developers before formal verification begins.

---

# Competitor 4

Security Audit Firms

Examples

OpenZeppelin

OtterSec

Zellic

MoveBit

Strengths

Industry trust.

Expert researchers.

Manual reasoning.

Business logic analysis.

Weaknesses

Expensive.

Late-stage.

Limited duration.

Cannot provide continuous feedback every time a developer saves a file.

MoveShield Position

Audits remain necessary.

MoveShield reduces the number of issues reaching auditors.

---

# Competitor 5

Research Tools

MoveScanner

Academic research demonstrates that static analysis for Move is feasible.

The proposed architecture combines

Control Flow Graphs

Data Flow Analysis

Cross-module analysis

Rule engines

The research reports strong detection performance but remains a research-oriented implementation rather than a mature developer platform.

MoveShield Position

Research proves feasibility.

MoveShield focuses on usability.

---

# Competitor 6

Manual Code Reviews

Still one of the most common quality assurance processes.

Strengths

Human reasoning.

Architecture discussion.

Business context.

Weaknesses

Time consuming.

Not repeatable.

Dependent on reviewer experience.

MoveShield Position

Reduce repetitive review work.

Allow reviewers to focus on architecture rather than obvious mistakes.

---

# Competitor Comparison

Compiler

Finds syntax.

MoveShield

Finds engineering problems.

---

Move Prover

Proves specifications.

MoveShield

Finds practical mistakes.

---

Audits

Human expertise.

MoveShield

Continuous automation.

---

Research Tools

Algorithms.

MoveShield

Developer experience.

---

Manual Review

Creative reasoning.

MoveShield

Repeatable automation.

---

# Market Gaps

The following gaps consistently appear across today's ecosystem.

Gap One

Continuous analysis.

Developers receive feedback too late.

---

Gap Two

Developer education.

Existing tools often explain what failed.

Few explain why.

---

Gap Three

Workflow integration.

Most tools operate independently.

Developers switch between multiple interfaces.

---

Gap Four

Engineering guidance.

Current tooling focuses on correctness.

Few tools recommend better engineering patterns.

---

Gap Five

Project quality scoring.

Developers cannot quickly understand the overall health of a Move project.

---

# What We Refuse To Compete On

We will never attempt to outperform

The compiler.

The verifier.

Professional auditors.

Formal verification.

Those systems already excel at their purpose.

Instead,

MoveShield fills the gaps between them.

---

# Our Position

MoveShield is not

A compiler.

A verifier.

An auditor.

A theorem prover.

MoveShield is

A continuous engineering assistant.

---

# Competitive Advantage

MoveShield differentiates itself through five principles.

Continuous analysis.

Developer-first UX.

Actionable explanations.

Workflow integration.

Extensible rule ecosystem.

No existing solution combines all five into one platform.

---

# Long-Term Vision

The long-term opportunity is larger than static analysis.

MoveShield should eventually become the engineering platform for Move.

Static Analysis

↓

Linting

↓

Security

↓

Performance

↓

Architecture

↓

CI/CD

↓

Developer Education

↓

Enterprise Governance

---

# Success Criteria

Five years from now,

we want developers to describe MoveShield like this.

"I wouldn't deploy a Move project without running MoveShield."

That is the benchmark.

Not GitHub stars.

Not downloads.

Developer trust.

---

# Conclusion

Competition validates demand.

It does not eliminate opportunity.

The Move ecosystem already contains excellent compilers, auditors, and verification tools.

What it lacks is a unified developer experience that brings continuous engineering quality into everyday development.

MoveShield exists to become that missing layer.