# RFC-0017

# Command Line Interface (CLI)

Status:
Draft

Version:
0.1.0

Target:
MoveShield CLI

---

# Abstract

The MoveShield Command Line Interface (CLI) is the primary user interface for interacting with the platform.

It provides a stable, discoverable, and scriptable interface for developers, CI systems, IDEs, and enterprise tooling.

The CLI is responsible for orchestration.

It never performs analysis itself.

---

# Design Goals

The CLI must be

Simple

Fast

Predictable

Composable

Script Friendly

CI Friendly

Backward Compatible

Cross Platform

---

# Philosophy

The CLI should feel like Cargo.

Developers should rarely need documentation to perform common tasks.

Commands should read naturally.

```
moveshield analyze
moveshield report
moveshield explain
moveshield graph
```

---

# Command Structure

```
moveshield

├── analyze
├── explain
├── report
├── graph
├── metrics
├── cache
├── plugins
├── config
├── doctor
├── init
├── version
└── help
```

---

# analyze

Runs a complete analysis.

Examples

```
moveshield analyze

moveshield analyze .

moveshield analyze ./contracts

moveshield analyze --policy strict
```

Output

Terminal summary

Optional report generation

Exit code

---

# explain

Displays documentation for a rule.

```
moveshield explain MS001
```

Output

Rule description

Examples

References

Suggested fixes

Severity

History

---

# report

Generates reports without rerunning analysis when cached findings exist.

Examples

```
moveshield report

moveshield report --format html

moveshield report --format sarif

moveshield report --output reports/
```

---

# graph

Displays engineering graphs.

Examples

```
moveshield graph call

moveshield graph dependency

moveshield graph capability

moveshield graph ownership
```

Future

GraphViz

Mermaid

Interactive HTML

---

# metrics

Displays engineering metrics.

Examples

```
moveshield metrics

moveshield metrics complexity

moveshield metrics api
```

Metrics include

Functions

Modules

Complexity

Documentation

Dependencies

Resources

Capabilities

---

# cache

Manages cached analysis.

Examples

```
moveshield cache status

moveshield cache clear

moveshield cache verify
```

---

# plugins

Plugin management.

Examples

```
moveshield plugins list

moveshield plugins install

moveshield plugins remove

moveshield plugins update
```

Future registry integration.

---

# config

Configuration utilities.

Examples

```
moveshield config show

moveshield config validate

moveshield config doctor
```

---

# doctor

Performs environment diagnostics.

Checks

Rust version

Move toolchain

Sui CLI

Configuration

Plugins

Cache

Filesystem permissions

---

# init

Creates a default configuration.

```
moveshield init
```

Produces

```
moveshield.toml
```

with recommended defaults.

---

# version

Displays

Engine Version

SDK Version

Rule Pack Version

Compiler Compatibility

Git Commit (development builds)

---

# Output Modes

Supported output

Human

JSON

Quiet

Verbose

Machine

Color output automatically adapts to terminal capabilities.

---

# Exit Codes

0

Success

1

Internal failure

2

Blocking findings

3

Configuration error

4

Plugin error

5

Environment error

Exit codes remain stable across releases.

---

# Global Options

Supported globally

```
--config

--policy

--threads

--format

--output

--no-cache

--verbose

--quiet

--json

--color

--help

--version
```

---

# Logging

Levels

Error

Warn

Info

Debug

Trace

Logging is separate from report generation.

---

# Performance

CLI startup should be minimal.

Configuration loading

Plugin discovery

Cache validation

should occur before expensive analysis begins.

---

# Accessibility

The CLI must

Support screen readers

Avoid color-only indicators

Provide plain text mode

Produce deterministic machine-readable output

---

# Future Commands

watch

serve

language-server

benchmark

audit

generate

publish

upgrade

Enterprise commands may extend the CLI through the Plugin SDK.

---

# Success Criteria

The CLI succeeds when

developers can perform common tasks intuitively,

automation is straightforward,

CI integration is reliable,

and future features fit naturally into the command hierarchy.

---

# Conclusion

The MoveShield CLI is the primary entry point into the platform.

By keeping the CLI focused on orchestration rather than analysis, MoveShield remains modular while providing a consistent developer experience across local development, continuous integration, and enterprise environments.