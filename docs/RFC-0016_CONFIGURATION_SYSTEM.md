# RFC-0016

# Configuration System

Status:
Draft

Version:
0.1.0

Target:
MoveShield Platform

---

# Abstract

The Configuration System defines how MoveShield is configured across local development, continuous integration, and enterprise environments.

Configuration must be predictable, versioned, and independent from the Move compiler.

MoveShield automatically discovers project configuration while allowing users and organizations to override defaults.

---

# Motivation

Every engineering organization has different requirements.

Some teams prioritize performance.

Some prioritize strict security.

Others require compliance or audit-focused analysis.

A flexible configuration system enables these use cases without modifying engine behavior.

---

# Design Goals

The configuration system must be

Deterministic

Versioned

Portable

Composable

Human Readable

Machine Readable

Backward Compatible

---

# Configuration File

The default configuration file is

```

moveshield.toml

```

MoveShield automatically searches from the current directory upward until the project root is found.

---

# Example

```toml
version = 1

[project]
name = "dex"

[analysis]
incremental = true
parallel = true
max_threads = 8

[policy]
profile = "strict"

[reports]
default = ["terminal", "sarif"]

[rules]
enabled = ["security", "performance", "architecture"]

[cache]
enabled = true

[plugins]
enabled = true
directory = ".moveshield/plugins"
```

---

# Configuration Sources

Configuration is loaded in this order.

Built-in defaults

↓

Global configuration

↓

Workspace configuration

↓

Project configuration

↓

Environment variables

↓

CLI arguments

The last value always wins.

---

# Built-in Defaults

Every installation contains safe defaults.

These guarantee that MoveShield runs without requiring a configuration file.

---

# Global Configuration

Optional user-level configuration.

Example location

```
~/.config/moveshield/config.toml
```

Typical uses

Default policy

Preferred reports

Plugin directory

Cache location

Telemetry preference

---

# Project Configuration

Project-specific settings stored in

```
moveshield.toml
```

Typical uses

Enabled rules

Severity overrides

Ignored paths

Policy profile

Plugin configuration

---

# Environment Variables

Supported for automation.

Examples

```
MOVESHIELD_CACHE_DIR

MOVESHIELD_POLICY

MOVESHIELD_THREADS

MOVESHIELD_LOG

MOVESHIELD_COLOR
```

Environment variables override configuration files.

---

# CLI Overrides

Command-line arguments have highest priority.

Example

```
moveshield analyze \
  --policy strict \
  --threads 16 \
  --report sarif
```

---

# Rule Configuration

Rules may expose configurable parameters.

Example

```toml
[rules.MS001]
max_complexity = 10

[rules.MS010]
allow_friend_modules = false
```

Unknown rule identifiers produce validation errors.

---

# Policy Configuration

Policies are referenced by identifier.

Example

```toml
[policy]
profile = "enterprise"
```

Future versions may support local policy bundles.

---

# Plugin Configuration

Each plugin owns its own configuration namespace.

Example

```toml
[plugins.graphviz]
enabled = true
format = "svg"
```

The engine never interprets plugin-specific options.

---

# Ignore Rules

Users may ignore files and directories.

Example

```toml
[ignore]
paths = [
    "build/",
    "generated/",
    "examples/"
]
```

Ignored files are excluded before analysis begins.

---

# Validation

Every configuration file is validated before execution.

Validation checks include

Schema version

Unknown keys

Invalid value types

Duplicate entries

Unsupported engine versions

Plugin compatibility

Validation errors stop execution before analysis.

---

# Versioning

Every configuration includes

Configuration Schema Version

Engine Compatibility

Policy Version

Future versions may include automatic migration tools.

---

# Diagnostics

MoveShield reports

Configuration source

Overridden values

Unknown keys

Deprecated settings

Migration suggestions

This improves debugging and user confidence.

---

# Security

Sensitive information should never be stored in configuration files.

Future enterprise versions may support

Encrypted secrets

External secret providers

Credential managers

---

# Testing

Configuration requires

Schema validation tests

Merge order tests

Environment override tests

CLI override tests

Regression tests

Migration tests

---

# Future Extensions

Remote configuration

Organization-wide configuration

Cloud-managed policies

Configuration profiles

Configuration templates

Live reload in IDEs

---

# Success Criteria

The Configuration System succeeds when

users can configure MoveShield consistently,

organizations can enforce standards,

and configuration remains stable across engine upgrades.

---

# Conclusion

The Configuration System provides a predictable and extensible foundation for controlling MoveShield.

By separating project, user, and organization settings while maintaining deterministic precedence rules, MoveShield remains flexible without sacrificing reproducibility.