## MoveShield Sui CLI

Download the archive for your operating system and CPU architecture, then
verify its SHA-256 digest against `SHA256SUMS` before extracting it.

This release's `moveshield` executable uses the official Sui Move compiler to
parse and type-check Sui package sources. The analysis model and security rules
remain incomplete; unsupported constructs fail analysis rather than producing
a partial report. This is not a full Sui build, bytecode verification, or a
security audit. A clean report does not prove a package is safe.

The CLI can resolve and fetch package dependencies when needed. Rust and a C
toolchain are not required to run the prebuilt binary.

See the README for install instructions, supported platforms, compiler
revision, and known limitations.
