# Sui Compiler Integration Handoff

## Status (2026-10-31)

- Repository: `Hacktod20/MoveShield`.
- Base: `main` at `d4a1c7c092819ed1ac7c115bd27c5aa6ef9b46f6` (PR #12 merged).
- Working branch: `feature/sui-unsupported-constructs`, based on that commit. PR [#13](https://github.com/Hacktod20/MoveShield/pull/13) is open and carries the follow-up validation work.
- Release/deployment: none. Keep the Sui backend unreleased; the default CLI and published `v0.1.0` binary remain unchanged.
- PR #12's latest reported Linux, macOS, Windows, and Sui compiler checks all passed. Those checks validate the merged baseline, not the changes on this branch.

## Implemented behavior

The opt-in CLI is isolated at `apps/sui-cli`, outside the root Cargo workspace. Its MystenLabs/Sui dependencies all use revision `79479aed266b8d2318ed91056aaf8f8a95f01ba5`; the standalone lockfile pins `allocative` to `0.3.4` to match the tested Sui dependency graph.

The adapter loads package dependencies, parses with the official compiler, type-checks through `PASS_TYPING`, and maps selected root-package modules, functions, structs, calls, complexity, and source locations into MoveShield's model. Loaded dependencies are type-checked but are not analyzed as MoveShield packages.

This branch changes previously silent omission of module-level enums, friend declarations, constants, specification blocks, macro functions, method-style `use fun` imports, and method-call expressions into an error containing the source path and position. The adapter returns no `Project`, so the CLI cannot emit an analysis report for those inputs. It reports the first unsupported construct in source order and stops. Independent fixtures cover each construct, exact locations, multiple unsupported members, and no stdout/files for terminal, JSON, SARIF, Markdown, HTML, and all output modes.

## Compatibility and coverage boundary

The only compiler revision claimed by this implementation is the pinned Sui commit above. Revision updates must be deliberate dependency and lockfile changes, followed by the Sui tests and CI; upstream `main` is not followed automatically. Package editions, flavors, dependency sources, lockfile mutations, offline behavior, and configuration support still need a formal tested matrix.

For the pinned compiler, [`PackageLoader::load`](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-package/src/package/package_loader.rs) documents that loading does not write the lockfile; writing pinned dependencies is a separate operation. A local no-dependency fixture now verifies the CLI leaves `Move.toml` byte-identical and does not create `Move.lock`. This does not yet establish mutation behavior for Git/local dependencies, repinning, offline failures, or other package layouts.

This is not a complete Sui build and does not run bytecode verification. Function bodies are retained as source text and summarized for calls/complexity; `Function.ast` is not populated with a complete MoveShield AST. Generic constraints, attributes, acquires, and other source/semantic details are not comprehensively represented. A successful compiler type check is not a security guarantee, and findings remain heuristic and incomplete.

## Research record

Accessed 2026-10-31:

- [Pinned Move parser AST](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-compiler/src/parser/ast.rs): `ModuleMember` includes `Function`, `Struct`, `Enum`, `Use`, `Friend`, `Constant`, and `Spec`. This exact source revision confirms the cases guarded by the adapter; it is the basis for failing closed rather than silently dropping the latter four.
- [Pinned Move lexer](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-compiler/src/parser/lexer.rs) and [edition definitions](https://github.com/MystenLabs/sui/blob/79479aed266b8d2318ed91056aaf8f8a95f01ba5/external-crates/move/crates/move-compiler/src/editions/mod.rs): `spec` and `invariant` tokens are enabled when the 2024 keyword feature is not; fixtures use `legacy` for specification syntax and `2024` for enums, macros, and method-style imports. These exact compiler details determine fixture manifests, not a broader package support claim.
- The pinned parser AST represents method-style imports as `Use::Fun`, macro declarations with `Function::macro_`, and calls as `Exp_::DotCall`. Those paths now fail closed because the MoveShield model does not preserve their macro semantics or reliably resolve their method-call targets.
- [Official Sui package overview](https://docs.sui.io/develop/write-move/package-overview): describes Sui's package/module model. It does not establish analyzer coverage or equate compiler acceptance with security.

## Validation and resources

- Installed the stable Rust toolchain from `rustup` (rustc 1.99.1, rustfmt, Clippy).
- Passed: `cargo fmt --manifest-path apps/sui-cli/Cargo.toml --all -- --check`.
- Passed: `cargo test --manifest-path apps/sui-cli/Cargo.toml --locked` (4 unit tests, 5 integration tests).
- Passed: focused unsupported-member location matrix, all-format no-report test, and multiple-member first-error test.
- Passed: `cargo clippy --manifest-path apps/sui-cli/Cargo.toml --locked --all-targets -- -D warnings`.
- The first clean Sui compilation took 11m38s; `apps/sui-cli/target` reached about 10 GB. At the last disk check, the workspace had 7.1 GB free. No root build was run because shared root crates were unchanged. Do not delete shared Cargo registry/Git caches.
- Before merging, verify required CI checks on PR #13's current head SHA. Local validation does not replace CI or complete the broader product gates below.
- Do not delete shared Cargo registry/Git caches. If cleanup is needed after a build, clean only this repository's root or `apps/sui-cli` target directories.

## Next work, in priority order

1. Commit and push the verified PR #13 follow-up, then check its latest Sui CI job. Acceptance: formatting, Sui tests, and Clippy all pass on the pushed SHA.
2. Audit the full pinned AST boundary, including `use` variants, attributes, generics, enum variants, function signatures/bodies, and source spans. Model supported data or fail with source-located errors; acceptance: fixtures prove each omitted construct is surfaced and no failed scan emits reports.
3. Establish and test package/toolchain behavior for supported editions, named addresses, local and Git dependencies, offline/missing-network cases, and workspace mutations. Acceptance: a documented support matrix and tests that assert failure does not emit reports or mutate user metadata unexpectedly.
4. Audit every advertised rule against Sui semantics and validate on a pinned, licensed public package corpus. Acceptance: rule-specific positive, negative, boundary, and regression fixtures plus explainable corpus results.
5. Complete CLI/report/configuration/SARIF schema gates, threat-model and limitations docs, performance baselines, and a release-readiness checklist. Keep release, publishing, deployment, and default-CLI migration out of scope without explicit approval.
