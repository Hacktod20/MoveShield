# MoveShield Static Analysis Platform

MoveShield is an experimental static analyzer for Sui Move packages. The
released `moveshield` CLI uses a pinned revision of the official Sui compiler
to parse and type-check packages before applying MoveShield's analysis rules.

MoveShield is available under either the [MIT License](LICENSE-MIT) or
[Apache License 2.0](LICENSE-APACHE), at your option.

> **Important limitation:** Compiler parsing and type checking do not make MoveShield's analysis complete. The model analyzes only selected root-package constructs, rejects unsupported constructs, does not verify bytecode, and its findings may be incomplete or incorrect. A clean scan is not evidence that a package is secure.

---

## Key Features

- **Decoupled Architecture:** Built on a modular Rust Cargo workspace with strict dependency hierarchies.
- **Official Sui compiler:** Resolves package dependencies and uses the pinned Sui compiler for parsing and type checking.
- **Advanced Graph Engine:** Assembles a fully resolved, immutable **Call Graph** with recursion and unused function detection.
- **Configurable Policies:** Implements extensible analysis rules, including **MS001 Cyclomatic Complexity**, governed by custom `moveshield.toml` settings.
- **Reports:** Prints terminal results and exports JSON, Markdown, HTML, and SARIF 2.1.0 (`./moveshield.sarif` by default).

---

## Workspace Overview

The workspace contains 15 supporting library crates:

| Crate Name | Primary Responsibility |
| :--- | :--- |
| [`moveshield-core`](crates/moveshield-core) | Canonical Core Data Model (CDM), traits, location info, and standard errors. |
| [`moveshield-compiler`](crates/moveshield-compiler) | Project discovery, Move.toml metadata, and the development parser adapter. |
| [`moveshield-ir`](crates/moveshield-ir) | Decoupled semantic intermediate representation for static analysis lookup trees. |
| [`moveshield-graph`](crates/moveshield-graph) | Immutable Call Graph construction, call site resolution, and cycles traversal. |
| [`moveshield-knowledge`](crates/moveshield-knowledge) | Engineering knowledge store housing clean, factual analysis stats. |
| [`moveshield-rules`](crates/moveshield-rules) | The static analysis scheduler and modular rules engine (e.g., MS001). |
| [`moveshield-policy`](crates/moveshield-policy) | Policy pipeline entry point; advanced filters and suppressions remain future work. |
| [`moveshield-findings`](crates/moveshield-findings) | Deduplicates and deterministically sorts platform findings. |
| [`moveshield-report`](crates/moveshield-report) | Outputs detailed terminal tables and exports standardized `moveshield.sarif`. |
| [`moveshield-config`](crates/moveshield-config) | Handles `moveshield.toml` loading, default configuration, and validation. |
| [`moveshield-sdk`](crates/moveshield-sdk) | SDK skeleton for future programmatic integrations. |
| [`moveshield-testing`](crates/moveshield-testing) | Shared testing mocks, harness fixtures, and regression pipelines. |
| [`moveshield-cache`](crates/moveshield-cache) | Cache skeleton; persistent incremental caching is not yet wired into the CLI. |
| [`moveshield-utils`](crates/moveshield-utils) | Standardized, non-business internal utility helpers. |

---

## Install and Quick Start

### Release and support status

The v0.1.2 release contains the compiler-backed `moveshield` CLI. The older
v0.1.1 release used a heuristic parser. The legacy heuristic CLI remains
available only as a source workspace tool; it is not the v0.1.2 release binary.

Release archives are built for GNU/Linux x86_64, macOS x86_64, macOS Apple
Silicon, and Windows x86_64. Ubuntu 24.04 x86_64 is the Linux environment
validated for installation. The release build checks that the Linux executable
does not require GLIBC symbols newer than 2.39; this does not establish a lower
minimum version, and other distributions are not yet claimed. macOS and Windows
release builds run in CI, but their clean-machine installation paths have not
been independently verified. Running a binary archive does not require Rust or
a C compiler; building from source requires the stable Rust toolchain and a
native C compiler.

Release archives and `SHA256SUMS` are downloaded from the same GitHub Releases
channel. The checksums detect accidental corruption; they are not a trusted
signature and do not authenticate an artifact if that channel is compromised.
MoveShield has no background updater, update check, or network activity during
analysis. For v1, check the [release page](https://github.com/Hacktod20/MoveShield/releases)
manually and install a newer archive using the verified steps below. This is a
manual workflow, not authenticated update delivery: the checksum and archive
come from the same unsigned release channel.

### Ubuntu 24.04 and GNU/Linux x86_64 with GLIBC 2.39 or newer

The commands install into `~/.local/bin` without `sudo`. They use `curl`,
`tar`, `grep`, and `sha256sum`.

```bash
set -euo pipefail
version=v0.1.2
target=x86_64-unknown-linux-gnu
archive="moveshield-${version}-${target}.tar.gz"
release="https://github.com/Hacktod20/MoveShield/releases/download/${version}"
mkdir -p "$HOME/Downloads/moveshield-$version" "$HOME/.local/bin"
cd "$HOME/Downloads/moveshield-$version"
curl -fLO "$release/$archive"
curl -fLO "$release/SHA256SUMS"
grep -F "  $archive" SHA256SUMS | sha256sum --check
tar -xzf "$archive" -C "$HOME/.local/bin"
chmod 755 "$HOME/.local/bin/moveshield"
touch "$HOME/.profile"
grep -qxF 'export PATH="$HOME/.local/bin:$PATH"' "$HOME/.profile" || \
	printf '%s\n' 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.profile"
export PATH="$HOME/.local/bin:$PATH"
moveshield --version
```

The prebuilt binary does not require Rust or Cargo. Sui package dependency
resolution may access the network when dependencies are not cached. Open a new
login shell after installation to load the saved `PATH` setting.

To upgrade, replace `v0.1.2` with the desired published tag and run:

```bash
set -euo pipefail
version=v0.1.2
target=x86_64-unknown-linux-gnu
archive="moveshield-${version}-${target}.tar.gz"
release="https://github.com/Hacktod20/MoveShield/releases/download/${version}"
mkdir -p "$HOME/Downloads/moveshield-$version"
cd "$HOME/Downloads/moveshield-$version"
curl -fLO "$release/$archive"
curl -fLO "$release/SHA256SUMS"
grep -F "  $archive" SHA256SUMS | sha256sum --check
tar -xzf "$archive" -C "$HOME/.local/bin"
chmod 755 "$HOME/.local/bin/moveshield"
moveshield --version
```

Extraction replaces the executable in place. To uninstall, run
`rm -f "$HOME/.local/bin/moveshield"` and remove the PATH line from
`~/.profile` if no other programs use that directory.

### macOS x86_64 and Apple Silicon

Use the archive matching the Mac. The commands below use macOS's built-in
`shasum`; the default shell is zsh.

```zsh
set -euo pipefail
version=v0.1.2
target=aarch64-apple-darwin # use x86_64-apple-darwin on Intel
archive="moveshield-${version}-${target}.tar.gz"
release="https://github.com/Hacktod20/MoveShield/releases/download/${version}"
mkdir -p "$HOME/Downloads/moveshield-$version" "$HOME/.local/bin"
cd "$HOME/Downloads/moveshield-$version"
curl -fLO "$release/$archive"
curl -fLO "$release/SHA256SUMS"
expected=$(grep -F "  $archive" SHA256SUMS | awk '{print $1}')
printf '%s  %s\n' "$expected" "$archive" | shasum -a 256 --check
tar -xzf "$archive" -C "$HOME/.local/bin"
chmod 755 "$HOME/.local/bin/moveshield"
touch "$HOME/.zprofile"
grep -qxF 'export PATH="$HOME/.local/bin:$PATH"' "$HOME/.zprofile" || \
	printf '%s\n' 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zprofile"
export PATH="$HOME/.local/bin:$PATH"
moveshield --version
moveshield analyze /path/to/move-project --format terminal
```

To upgrade, replace `v0.1.2` with the desired published tag and run:

```zsh
set -euo pipefail
version=v0.1.2
target=aarch64-apple-darwin # use x86_64-apple-darwin on Intel
archive="moveshield-${version}-${target}.tar.gz"
release="https://github.com/Hacktod20/MoveShield/releases/download/${version}"
mkdir -p "$HOME/Downloads/moveshield-$version"
cd "$HOME/Downloads/moveshield-$version"
curl -fLO "$release/$archive"
curl -fLO "$release/SHA256SUMS"
expected=$(grep -F "  $archive" SHA256SUMS | awk '{print $1}')
printf '%s  %s\n' "$expected" "$archive" | shasum -a 256 --check
tar -xzf "$archive" -C "$HOME/.local/bin"
chmod 755 "$HOME/.local/bin/moveshield"
moveshield --version
```

To uninstall, run `rm -f "$HOME/.local/bin/moveshield"` and remove the PATH
line from `~/.zprofile` if it is no longer needed.

### Windows x86_64

Run these commands in PowerShell. Windows includes `tar.exe`; no Rust toolchain
is required.

```powershell
$version = 'v0.1.2'
$target = 'x86_64-pc-windows-msvc'
$archive = "moveshield-$version-$target.tar.gz"
$release = "https://github.com/Hacktod20/MoveShield/releases/download/$version"
$download = Join-Path $HOME "Downloads\MoveShield\$version"
$install = Join-Path $HOME 'AppData\Local\Programs\MoveShield'
New-Item -ItemType Directory -Force $download, $install | Out-Null
Set-Location $download
Invoke-WebRequest "$release/$archive" -OutFile $archive
Invoke-WebRequest "$release/SHA256SUMS" -OutFile SHA256SUMS
$line = (Select-String -Path SHA256SUMS -SimpleMatch "  $archive").Line
if (-not $line) { throw "No checksum found for $archive" }
$expected = ($line -split '\s+')[0].ToLowerInvariant()
$actual = (Get-FileHash $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actual -ne $expected) { throw 'SHA-256 verification failed' }
tar.exe -xzf $archive -C $install
if ($LASTEXITCODE -ne 0) { throw 'Archive extraction failed' }
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if (($userPath -split ';') -notcontains $install) {
	$updatedPath = if ($userPath) { "$userPath;$install" } else { $install }
	[Environment]::SetEnvironmentVariable('Path', $updatedPath, 'User')
}
$env:Path = "$install;$env:Path"
moveshield.exe --version
moveshield.exe analyze C:\path\to\move-project --format terminal
```

For an upgrade, replace `v0.1.2` with the desired published tag and run:

```powershell
$version = 'v0.1.2'
$target = 'x86_64-pc-windows-msvc'
$archive = "moveshield-$version-$target.tar.gz"
$release = "https://github.com/Hacktod20/MoveShield/releases/download/$version"
$download = Join-Path $HOME "Downloads\MoveShield\$version"
$install = Join-Path $HOME 'AppData\Local\Programs\MoveShield'
New-Item -ItemType Directory -Force $download, $install | Out-Null
Set-Location $download
Invoke-WebRequest "$release/$archive" -OutFile $archive
Invoke-WebRequest "$release/SHA256SUMS" -OutFile SHA256SUMS
$line = (Select-String -Path SHA256SUMS -SimpleMatch "  $archive").Line
if (-not $line) { throw "No checksum found for $archive" }
$expected = ($line -split '\s+')[0].ToLowerInvariant()
$actual = (Get-FileHash $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actual -ne $expected) { throw 'SHA-256 verification failed' }
tar.exe -xzf $archive -C $install
if ($LASTEXITCODE -ne 0) { throw 'Archive extraction failed' }
moveshield.exe --version
```

Extraction replaces the existing executable. To uninstall, run
`Remove-Item "$HOME\AppData\Local\Programs\MoveShield\moveshield.exe"` and
remove that directory from your user `Path` if it is no longer used.

### Analyze a Move project

```bash
moveshield analyze ./example --format terminal
moveshield analyze ./example --format json
moveshield analyze ./example --format sarif --output ./moveshield.sarif
moveshield analyze ./example --format markdown --output ./report.md
moveshield analyze ./example --format html --output ./report.html
```

The bundled `./example` scan reports findings and exits `2`; this is not an
installation or runtime failure. Analysis exits `0` when no findings are
reported, `1` for analysis or report failures, `2` when findings exist, and `3`
for configuration errors. JSON is written to stdout unless `--output` is set.
The default format prints a terminal report and writes `moveshield.sarif` in the
current directory. A clean report is not evidence that a package is secure.

The CLI type-checks the package with the official Sui compiler. Dependencies
are resolved for type checking, but MoveShield findings are currently derived
from supported constructs in the root package only. Unsupported enums, friend
declarations, constants, specifications, macro forms, method-style imports,
and method calls fail analysis with a source-located error rather than
producing a partial report. This is not a full Sui build or bytecode verifier.

### Build and test from source

```bash
cargo build --locked --release --manifest-path apps/sui-cli/Cargo.toml --bin moveshield
cargo test --locked --manifest-path apps/sui-cli/Cargo.toml
```

Building from source requires stable Rust, a native C compiler/linker, and
network access to fetch the pinned Sui compiler dependencies. The regular
workspace CLI can still be run separately for development, but it uses the
heuristic parser and is not the release product.

---

## Comprehensive Documentation

Detailed architecture specifications, developer onboarding, rules specifications, and ADR logs can be accessed directly in the `docs/` workspace:

- **[System Architecture & Scan Pipeline](docs/ARCHITECTURE.md):** Deep-dive into overall scan pipeline diagrams, Mermaid flows, CDM details, ADR logs, and guide on how to add custom analysis rules.
- **[Configuration & Developer Setup Guide](docs/DEVELOPER_GUIDE.md):** Detailed reference for `moveshield.toml`, fresh OS/dependency setups, testing guidelines, and troubleshooting.
- **[Founding Principles](docs/0000_FOUNDING_PRINCIPLES.md):** Engineering kickoff guide outlining MoveShield's long-term design visions and architectural milestones.
