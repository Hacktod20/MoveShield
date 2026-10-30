## MoveShield CLI

Download the archive for your operating system and CPU architecture, then
verify its SHA-256 digest against `SHA256SUMS` before extracting it.

### Important analysis limitation

This release uses MoveShield's heuristic development parser. It does not use
the official Sui or Aptos compiler AST, does not type-check packages, and can
produce incomplete or incorrect findings. Do not rely on it as a security
audit or treat a clean scan as proof that a package is safe.

See the README and project status report for supported commands, setup, and
known limitations.
