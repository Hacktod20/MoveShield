# Sui Compiler Integration Handoff

## Status (2026-11-01)

- Repository: `Hacktod20/MoveShield`.
- Base: `main` at `d4a1c7c092819ed1ac7c115bd27c5aa6ef9b46f6` (PR #12 merged).
- Working branch: `feature/sui-unsupported-constructs`, based on that commit. PR [#13](https://github.com/Hacktod20/MoveShield/pull/13) is open and carries the follow-up validation work.
- Published releases through v0.1.1 contain only the heuristic CLI. v0.1.2 is being prepared to ship the compiler-backed CLI as the default `moveshield`; do not publish until the target-platform build and release smoke checks pass.
- At review time, PR #13 head `6c75ba0` had passing Linux, macOS, Windows, and Sui compiler checks. Follow-up changes in this worktree require a fresh CI run before merge.

## Implemented behavior

The Sui CLI is isolated at `apps/sui-cli`, outside the root Cargo workspace. Its MystenLabs/Sui dependencies all use revision `79479aed266b8d2318ed91056aaf8f8a95f01ba5`; the standalone lockfile pins `allocative` to `0.3.4` to match the tested Sui dependency graph. The v0.1.2 release candidate names its executable `moveshield`.

The adapter loads package dependencies, parses with the official compiler, type-checks through `PASS_TYPING`, and maps selected root-package modules, functions, structs, calls, complexity, and source locations into MoveShield's model. Loaded dependencies are type-checked but are not analyzed as MoveShield packages.

The adapter rejects module-level enums, friend declarations, constants, specification blocks, macro functions, method-style `use fun` imports, method-call expressions, macro-call expressions, and function-body specification expressions with a source-located error. The adapter returns no `Project`, so the CLI cannot emit an analysis report for those inputs. It reports the first unsupported construct in source order and stops. Independent fixtures cover each construct, exact locations, multiple unsupported members, and no stdout/files for terminal, JSON, SARIF, Markdown, HTML, and all output modes.

## Compatibility and coverage boundary

The only compiler revision claimed by this implementation is the pinned Sui commit above. Revision updates must be deliberate dependency and lockfile changes, followed by the Sui tests and CI; upstream `main` is not followed automatically. Package editions, flavors, dependency sources, lockfile mutations, offline behavior, and configuration support still need a formal tested matrix.

For the pinned compiler, [`PackageLoader::load`](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-package/src/package/package_loader.rs) documents that loading does not write the lockfile; writing pinned dependencies is a separate operation. Local fixtures with and without a local dependency verify the CLI leaves the root `Move.toml` byte-identical and does not create `Move.lock`. This does not yet establish mutation behavior for Git dependencies, repinning, offline failures, or other package layouts.

A separate local-dependency fixture verifies a `local = "../helper"` dependency resolves and type-checks, while the MoveShield report contains only the root module. Git dependencies, nested/transitive dependency graphs, dependency address remapping, and offline/missing-network behavior remain untested.

This is not a complete Sui build and does not run bytecode verification. Function bodies are retained as source text and summarized for calls/complexity; `Function.ast` is not populated with a complete MoveShield AST. Generic constraints, attributes, acquires, and other source/semantic details are not comprehensively represented. A successful compiler type check is not a security guarantee, and findings remain heuristic and incomplete.

## Release installation and update decision

Published v0.1.0 and v0.1.1 binaries use the heuristic development parser. The v0.1.2 candidate switches the default release executable to the compiler-backed Sui analyzer. The release workflow packages raw `.tar.gz` archives for GNU/Linux x86_64, macOS Intel/Apple Silicon, and Windows x86_64; it does not create OS packages, an installer, an uninstaller, an updater, or signed release metadata.

Accessed 2026-11-01:

- [Release workflow](../.github/workflows/release.yml) and [published v0.1.0 assets](https://github.com/Hacktod20/MoveShield/releases/tag/v0.1.0): the archive and `SHA256SUMS` are distributed as assets of the same GitHub release.
- [GitHub Releases documentation](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases): release assets and notifications are available, but this repository's current checksum file is not a signature.
- Inspected the released Linux binary with `ldd` and `readelf`: it requires `libc.so.6`, `libgcc_s.so.1`, and GLIBC symbols through 2.39.

The v1 update decision is explicit manual checking of GitHub Releases and repeating the verified download/extraction steps in the README. There is no network request during analysis and no self-update path. SHA-256 catches corruption but cannot authenticate an artifact when the checksum and binary share an unsigned channel; do not describe the current process as signature verification. Signed provenance or a maintained package repository should precede stronger trust claims. No tag, release, package, deployment, or default-CLI migration is part of this follow-up.

The README install flow and smoke script were tested on Ubuntu 24.04 x86_64 (GLIBC 2.39) against the actual v0.1.0 Linux release archive. The test ran in a clean `ubuntu:24.04` container as UID 1000, with no Rust toolchain or privilege elevation; checksum, extraction, permissions, PATH lookup, `--version`, the documented missing-tool `doctor` result, an actual analysis, binary replacement, and uninstall all passed. Older Linux distributions and Debian are not claimed. macOS and Windows have release build jobs, but their installation flows have not been run on clean native machines.

## Research record

Accessed 2026-11-01:

- [Pinned Move parser AST](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-compiler/src/parser/ast.rs): `ModuleMember` includes `Function`, `Struct`, `Enum`, `Use`, `Friend`, `Constant`, and `Spec`. This exact source revision confirms the cases guarded by the adapter; it is the basis for failing closed rather than silently dropping the latter four.
- [Pinned Move lexer](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-compiler/src/parser/lexer.rs) and [edition definitions](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-compiler/src/editions/mod.rs): `spec` and `invariant` tokens are enabled when the 2024 keyword feature is not; fixtures use `legacy` for specification syntax and `2024` for enums, macros, and method-style imports. These exact compiler details determine fixture manifests, not a broader package support claim.
- The pinned parser AST represents method-style imports as `Use::Fun`, macro declarations with `Function::macro_`, macro invocations as `Exp_::Call` with `NameAccess::is_macro`, method calls as `Exp_::DotCall`, and specification expressions as `Exp_::Spec`. These paths fail closed because the MoveShield model does not preserve their semantics.
- [Official Sui package overview](https://docs.sui.io/develop/write-move/package-overview): describes Sui's package/module model. It does not establish analyzer coverage or equate compiler acceptance with security.

## Validation and resources

- Installed the stable Rust toolchain from the checksum-verified official `rustup-init` binary (rustc 1.99.1, rustfmt, Clippy); the installer did not modify shell startup files.
- Passed: `cargo fmt --manifest-path apps/sui-cli/Cargo.toml --all -- --check`.
- Passed: `cargo test --manifest-path apps/sui-cli/Cargo.toml --locked` (4 unit tests, 6 integration tests).
- Passed: `cargo clippy --manifest-path apps/sui-cli/Cargo.toml --locked --all-targets --quiet -- -D warnings`.
- Passed: the unsupported-construct exact-location matrix and all-format no-report tests, including the new macro-call and function-body spec-expression fixtures.
- Passed: `bash scripts/smoke-linux-install.sh ...` against the actual v0.1.0 archive, both on the toolchain-enabled host and in a clean Ubuntu 24.04 container as an unprivileged user.
- Before Sui compilation, `/workspaces` had 19 GB free; after the full tests and Clippy, `apps/sui-cli/target` was 12 GB and 5.2 GB remained free. Do not delete shared Cargo registry/Git caches.
- Added a Linux release-install smoke step to CI. It has not yet run on the changed PR head; require fresh checks after the changes are pushed. Local validation does not replace CI or complete the broader product gates below.
- Do not delete shared Cargo registry/Git caches. If cleanup is needed after a build, clean only this repository's root or `apps/sui-cli` target directories.

## Next work, in priority order

1. Commit and push the verified PR #13 follow-up, then check its latest Sui CI job. Acceptance: formatting, Sui tests, and Clippy all pass on the pushed SHA.
2. Audit the full pinned AST boundary, including `use` variants, attributes, generics, enum variants, function signatures/bodies, and source spans. Model supported data or fail with source-located errors; acceptance: fixtures prove each omitted construct is surfaced and no failed scan emits reports.
3. Establish and test package/toolchain behavior for supported editions, named addresses, local and Git dependencies, offline/missing-network cases, and workspace mutations. Acceptance: a documented support matrix and tests that assert failure does not emit reports or mutate user metadata unexpectedly.
4. Audit every advertised rule against Sui semantics and validate on a pinned, licensed public package corpus. Acceptance: rule-specific positive, negative, boundary, and regression fixtures plus explainable corpus results.
5. Complete CLI/report/configuration/SARIF schema gates, threat-model and limitations docs, performance baselines, and a release-readiness checklist. Keep release, publishing, deployment, and default-CLI migration out of scope without explicit approval.
