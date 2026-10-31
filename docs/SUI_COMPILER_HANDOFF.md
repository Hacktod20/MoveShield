# Sui Compiler Integration Handoff

## Status (2026-10-31)

- Repository: `Hacktod20/MoveShield`.
- Base: `main` at `d4a1c7c092819ed1ac7c115bd27c5aa6ef9b46f6` (PR #12 merged).
- Working branch: `feature/sui-unsupported-constructs`, created from that base. Changes are not committed or pushed; no follow-up PR exists yet.
- Release/deployment: none. Keep the Sui backend unreleased; the default CLI and published `v0.1.0` binary remain unchanged.
- PR #12's latest reported Linux, macOS, Windows, and Sui compiler checks all passed. Those checks validate the merged baseline, not the changes on this branch.

## Implemented behavior

The opt-in CLI is isolated at `apps/sui-cli`, outside the root Cargo workspace. Its MystenLabs/Sui dependencies all use revision `79479aed266b8d2318ed91056aaf8f8a95f01ba5`; the standalone lockfile pins `allocative` to `0.3.4` to match the tested Sui dependency graph.

The adapter loads package dependencies, parses with the official compiler, type-checks through `PASS_TYPING`, and maps selected root-package modules, functions, structs, calls, complexity, and source locations into MoveShield's model. Loaded dependencies are type-checked but are not analyzed as MoveShield packages.

This branch changes previously silent omission of module-level enums, friend declarations, constants, and specification blocks into an error containing the source path and position. The adapter returns no `Project`, so the CLI cannot emit an analysis report for those inputs. A regression test covers a constant. The change remains unverified locally because this container has no `cargo` executable.

## Compatibility and coverage boundary

The only compiler revision claimed by this implementation is the pinned Sui commit above. Revision updates must be deliberate dependency and lockfile changes, followed by the Sui tests and CI; upstream `main` is not followed automatically. Package editions, flavors, dependency sources, lockfile mutations, offline behavior, and configuration support still need a formal tested matrix.

This is not a complete Sui build and does not run bytecode verification. Function bodies are retained as source text and summarized for calls/complexity; `Function.ast` is not populated with a complete MoveShield AST. Generic constraints, attributes, acquires, and other source/semantic details are not comprehensively represented. A successful compiler type check is not a security guarantee, and findings remain heuristic and incomplete.

## Research record

Accessed 2026-10-31:

- [Pinned Move parser AST](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-compiler/src/parser/ast.rs): `ModuleMember` includes `Function`, `Struct`, `Enum`, `Use`, `Friend`, `Constant`, and `Spec`. This exact source revision confirms the cases guarded by the adapter; it is the basis for failing closed rather than silently dropping the latter four.
- [Official Sui package overview](https://docs.sui.io/develop/write-move/package-overview): describes Sui's package/module model. It does not establish analyzer coverage or equate compiler acceptance with security.

## Validation and resources

- Attempted: `cargo test --manifest-path apps/sui-cli/Cargo.toml --locked unsupported_module_members_fail_with_source_location`. It did not start: `bash: cargo: command not found`.
- Not run: Rust formatting, Sui tests, Clippy, and root workspace checks. No build was started. At inspection, the workspace filesystem had 19 GB free and no repository `target` directory was present.
- Historical PR #12 check results are successful, but cannot validate this branch's edits. Do not treat this change as verified until focused Sui tests and Sui CI pass.
- Do not delete shared Cargo registry/Git caches. If cleanup is needed after a build, clean only this repository's root or `apps/sui-cli` target directories.

## Next work, in priority order

1. Make Rust tooling available, then run `cargo fmt --manifest-path apps/sui-cli/Cargo.toml --all -- --check`, the focused unsupported-member test, the full locked Sui tests, and Sui Clippy. Acceptance: all pass on this branch or CI, with resource use recorded.
2. Add independent fixtures for enum, friend, and specification diagnostics, and then model or explicitly surface other unsupported AST constructs. Acceptance: every dropped compiler node is represented or prevents report generation with a mapped source location.
3. Establish and test package/toolchain behavior for supported editions, named addresses, local and Git dependencies, offline/missing-network cases, and workspace mutations. Acceptance: a published support matrix and tests that assert failure does not emit reports or mutate user metadata unexpectedly.
4. Audit every advertised rule against Sui semantics and validate on a pinned, licensed public package corpus. Acceptance: rule-specific positive, negative, boundary, and regression fixtures plus explainable corpus results.
5. Complete CLI/report/configuration/SARIF gates, threat-model and limitations docs, performance baselines, and a release-readiness checklist. Keep release, publishing, deployment, and default-CLI migration out of scope without explicit approval.
