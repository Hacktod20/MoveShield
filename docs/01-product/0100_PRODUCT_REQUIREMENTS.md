# MoveShield

# 0100 — Product Requirements Document (PRD)

Version: 0.1.0

Status:
Draft

Author:
Muiz

Last Updated:
July 2026

---

# Table of Contents

1. Executive Summary
2. Product Overview
3. Product Vision
4. Problem Statement
5. Product Objectives
6. Target Users
7. User Stories
8. Functional Requirements
9. Non-Functional Requirements
10. Product Scope
11. Out of Scope
12. Success Metrics
13. Product Roadmap

---

# Executive Summary

MoveShield is a developer-first engineering platform for Move smart contracts.

Its purpose is to continuously improve software quality before contracts reach production.

Rather than replacing compilers, auditors, or formal verification, MoveShield connects them into a single workflow by providing static analysis, engineering guidance, code quality checks, security insights, and developer education.

The first release focuses on a fast local CLI.

Future releases expand into IDE integration, GitHub automation, cloud collaboration, and enterprise governance.

---

# Product Overview

MoveShield consists of four major products.

1.

Core Analysis Engine

Responsible for parsing, analyzing, and evaluating Move projects.

---

2.

Developer CLI

Allows local analysis from the command line.

Example

moveshield scan

---

3.

Developer Integrations

VS Code

GitHub Actions

CI/CD

---

4.

Cloud Platform

Project history

Organizations

Dashboards

Team collaboration

API

Billing

---

# Product Vision

Every serious Move project should use MoveShield before deployment.

The ideal workflow becomes

Write

↓

Save

↓

MoveShield

↓

Fix

↓

Commit

↓

Deploy

Quality becomes continuous.

---

# Problem Statement

Current Move development relies on disconnected tools.

Compiler

↓

Tests

↓

Manual Review

↓

Audit

↓

Production

There is no unified quality platform that continuously assists developers throughout development.

MoveShield solves this problem.

---

# Product Objectives

Primary Objective

Reduce engineering mistakes before deployment.

Secondary Objectives

Improve developer productivity.

Reduce audit preparation time.

Improve code consistency.

Encourage engineering best practices.

Create educational feedback.

Integrate naturally into existing workflows.

---

# Target Users

Primary

Individual Move developers.

---

Secondary

Startups building on Sui.

---

Enterprise

Large blockchain engineering teams.

---

Educational

Universities.

Bootcamps.

Training programs.

---

Security

Audit firms.

Researchers.

Protocol engineers.

---

# User Stories

As a developer

I want to detect security issues before deployment

So that I reduce risk.

---

As a developer

I want actionable explanations

So I understand why the issue exists.

---

As a team lead

I want consistent quality checks

So every pull request follows engineering standards.

---

As an auditor

I want projects to arrive with fewer obvious mistakes

So I can focus on deeper architectural analysis.

---

# Functional Requirements

The system shall:

Compile Move projects using the official toolchain.

Analyze project structure.

Inspect modules.

Inspect functions.

Inspect capabilities.

Inspect visibility.

Inspect ownership.

Inspect resources.

Inspect events.

Generate findings.

Rank findings by severity.

Generate reports.

Export SARIF.

Export JSON.

Export HTML.

Support configuration files.

Support custom rules.

Support rule disabling.

Support rule severity customization.

Support incremental analysis.

Support multi-package projects.

---

# CLI Requirements

Commands

moveshield scan

moveshield init

moveshield doctor

moveshield explain

moveshield rules

moveshield config

moveshield version

moveshield update

Every command must produce deterministic output.

---

# Report Requirements

Terminal

JSON

SARIF

Markdown

HTML

Future

PDF

Interactive Dashboard

---

# Rule Requirements

Every rule must define

Identifier

Name

Description

Purpose

Severity

Confidence

Category

Examples

Suggested Fix

References

Rules must remain independent.

---

# Performance Requirements

Cold startup should be fast.

Typical projects should complete analysis within a few seconds.

Memory usage should remain predictable.

Analysis should support parallel execution where practical.

The platform should scale to large codebases without requiring architectural changes.

---

# Reliability Requirements

Deterministic results.

Stable rule execution.

Graceful failure handling.

Recoverable diagnostics.

Comprehensive testing.

---

# Security Requirements

No telemetry by default.

Offline execution.

No source code uploaded without explicit consent.

Configuration files remain local.

Developer privacy is respected.

---

# Accessibility Requirements

Readable terminal output.

Color-blind friendly severity indicators.

Machine-readable reports.

Clear documentation.

---

# Product Scope

Version 1

CLI

Rule Engine

Static Analysis

JSON

SARIF

HTML

Configuration

Custom Rules

---

Version 2

VS Code

GitHub Action

Language Server

---

Version 3

Cloud

Organizations

Projects

History

API

Billing

---

Version 4

Enterprise

Policies

Compliance

Governance

Analytics

---

# Out of Scope

MoveShield is not

A compiler.

A blockchain.

A wallet.

A deployment tool.

A package manager.

A node.

A testing framework.

A replacement for professional audits.

---

# Success Metrics

Version 1

100 GitHub stars.

50 weekly users.

10 community contributors.

---

Version 2

500 active projects.

100 GitHub Action installations.

---

Version 3

100 paying teams.

---

Long Term

Become a standard dependency in serious Move projects.

---

# Product Roadmap

Milestone 1

Compiler integration.

Milestone 2

Analysis engine.

Milestone 3

Rule engine.

Milestone 4

CLI.

Milestone 5

Report generation.

Milestone 6

GitHub Action.

Milestone 7

VS Code extension.

Milestone 8

Cloud Platform.

Milestone 9

Enterprise Platform.

---

# Product Principles

Every feature must satisfy these questions.

Does it improve developer confidence?

Does it improve software quality?

Does it simplify engineering?

Does it integrate naturally?

Can it be maintained for years?

If the answer is no, the feature should be reconsidered.

---

# Definition of Success

MoveShield succeeds when developers stop asking

"Should I run MoveShield?"

and instead ask

"Why haven't I run MoveShield yet?"

At that point, MoveShield has become part of the normal Move development lifecycle.