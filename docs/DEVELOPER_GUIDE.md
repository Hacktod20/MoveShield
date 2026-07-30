# MoveShield Configuration and Developer Onboarding Guide

---

## 1. `moveshield.toml` Reference Guide

Every Move project scanned by MoveShield can declare a `moveshield.toml` file in its root directory to customize thresholds and control rules. If the file is absent, standard platform defaults are applied.

### 1.1 Complete Configuration Reference

Here is a full TOML example customizing the default rules:

```toml
# Schema version of the configuration file. Currently, the default is 1.
version = 1

[rules.MS001]
# Enable or disable the Cyclomatic Complexity rule (default is true).
enabled = true

# The threshold for maximum allowed cyclomatic complexity inside a single function body.
# If a function exceeds this threshold, a Medium severity finding is generated.
# Default threshold is 10.
max_complexity = 10
```

### 1.2 Configuration Fields Specification

| Field Name | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `version` | Integer | `1` | Schema version of the configuration file. |
| `rules.MS001.enabled` | Boolean | `true` | When true, the MS001 Cyclomatic Complexity check is run. |
| `rules.MS001.max_complexity` | Integer | `10` | The exclusive maximum allowable cyclomatic complexity score. |

---

## 2. Developer Setup & Onboarding (Fresh Install)

MoveShield is built in Rust using a modular Cargo workspace architecture. Follow these instructions to set up a clean development environment.

### 2.1 System Dependencies & Tooling
Before compiling MoveShield, verify you have the following installed:
- **Git:** Required to clone and manage codebase branches.
- **Rust Toolchain:** MoveShield requires Rust stable edition. The workspace automatically pins the correct stable version via `rust-toolchain.toml`.

### 2.2 Installing the Rust Toolchain
If you do not have Rust installed, set it up via `rustup`:

```bash
# Download and install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure your shell
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2.3 Cloning the Project
Clone the repository recursively:

```bash
git clone https://github.com/user/moveshield.git
cd moveshield
```

### 2.4 Building the Workspace
To compile the entire workspace (including applications and internal crates):

```bash
# Build the workspace in debug mode
cargo build

# Build the workspace in release mode (optimized)
cargo build --release
```

---

## 3. Running the CLI against the Example Project

MoveShield includes a dedicated example Move package located under `example/`.

### 3.1 Analyzing the Example Project
You can execute the MoveShield CLI directly against the example directory:

```bash
# Run the scanner against the example project
cargo run --package moveshield-cli -- analyze ./example
```

### 3.2 Example Modules & Behaviors Demonstrated

The `example/` project contains two Move modules in `example/sources/` specifically written to demonstrate compiler adapter parsing and rule violations:

1. **`clean.move` (Module: `example::clean`):**
   - **Purpose:** Demonstrates highly maintainable, modular Move code.
   - **Functions:**
     - `get_base_fee()`: Simple getter (Complexity = 1).
     - `calculate_fee(amount)`: Calculates fee (Complexity = 1).
   - **Analysis Outcome:** Passes with zero findings.

2. **`complex.move` (Module: `example::complex`):**
   - **Purpose:** Demonstrates deep branching and inter-module execution flows.
   - **Functions:**
     - `transfer(...)`: A multi-branched function containing ten consecutive `if` statements and logical `&&` checks.
   - **Analysis Outcome:** Triggers the **MS001 Cyclomatic Complexity** violation because its cyclomatic complexity is calculated at **12**, which is higher than the allowed configuration maximum threshold of **10**.

### 3.3 CLI Exit Codes

The CLI returns predictable exit codes depending on the scanning results to integrate smoothly into CI/CD pipelines:
- **`0`**: Scan succeeded, project analysed, and **no blocking findings** were detected.
- **`1`**: **Internal Error** (e.g., project compile error, missing sources, write failure).
- **`2`**: **Blocking Findings Detected** (findings exceeding thresholds were successfully discovered during the scan).
- **`3`**: **Configuration Error** (invalid or malformed `moveshield.toml` file).

---

## 4. Testing Strategy Guide

MoveShield enforces a disciplined testing strategy. No code is committed to main branches without corresponding unit or integration test coverage.

### 4.1 Running the Complete Test Suite
To execute all unit and integration tests across every workspace crate:

```bash
cargo test
```

### 4.2 Running Specific Crate Tests
Since MoveShield is organized into micro-crates, you can test specific layers independently:

```bash
# Test only the Compiler Adapter and Parser fallback
cargo test -p moveshield-compiler

# Test only the Graph Engine & CallGraph builders
cargo test -p moveshield-graph

# Test only configuration TOML parsers
cargo test -p moveshield-config
```

### 4.3 Testing Coverage
- **Parser & Adapter Tests:** Validate exact parsing of module definitions, structures, visibility modifiers (`public`, `entry`, etc.), comments stripping, and inter-module function calls.
- **Graph & CallGraph Tests:** Validate recursive path detection, call site resolution, unused function calculation, and cycle traversing.
- **Rules Tests:** Ensure that rule thresholds trigger findings accurately depending on customizable configurations.

---

## 5. Troubleshooting & FAQ

#### Q: How do I run MoveShield when `Move.toml` is absent?
A: MoveShield's discovery phase requires a valid `Move.toml` inside the target directory. If compiling a standalone `.move` file, wrap it in a minimal package containing a `Move.toml` structure.

#### Q: I modified the `example/moveshield.toml` rule thresholds but the CLI exit code didn't change?
A: If you increase the `max_complexity` threshold to `15`, the `complex::transfer` function complexity (which is 12) will be below the threshold, and the scan will pass with exit code `0`. Try testing this behavior!
