# MoveShield Security Rules Catalog (Milestone 2)

This document catalogs every security, performance, and architectural rule implemented in MoveShield. Each section details the rule purpose, detection logic, false-positive considerations, and remediation guidance.

---

## Catalog Overview

| Rule ID | Name | Category | Default Severity | CWE Reference |
|---|---|---|---|---|
| [MS001](#ms001) | Cyclomatic Complexity | Maintainability | Medium | - |
| [MS002](#ms002) | Public Transfer Detection | Security | High | CWE-284 |
| [MS003](#ms003) | Capability Leakage | Security | High | CWE-285 |
| [MS004](#ms004) | Shared Object Misuse | Security | High | CWE-284 |
| [MS005](#ms005) | Missing Access Control | Security | Critical | CWE-285 |
| [MS006](#ms006) | Timestamp / Clock Misuse | Security | Medium | CWE-829 |
| [MS007](#ms007) | Dynamic Field Risks | Security | Medium | CWE-252 |
| [MS008](#ms008) | Upgrade Policy Issues | Security | High | CWE-285 |
| [MS009](#ms009) | Treasury Misconfiguration | Security | High | CWE-284 |
| [MS010](#ms010) | Object Ownership Violations | Security | Medium | CWE-284 |
| [MS011](#ms011) | Randomness Misuse | Security | High | CWE-337 |
| [MS012](#ms012) | Gas Inefficiencies | Performance | Informational | CWE-400 |
| [MS013](#ms013) | Unused Capabilities | Architecture | Medium | CWE-561 |
| [MS014](#ms014) | Dead Public Functions | Architecture | Medium | CWE-561 |
| [MS015](#ms015) | Duplicate Objects | Architecture | Medium | - |

---

## Rule Details

### MS001

* **Name**: Cyclomatic Complexity
* **Category**: `Maintainability`
* **Default Severity**: `Medium`
* **CWE**: None
* **Description**: Detects functions with high cyclomatic complexity that are hard to understand, maintain, and test.
* **Detection Logic**: Iterates through all functions in the project and calculates a complexity score based on conditional statements (`if`, `while`, `loop`) and logical operators (`&&`, `||`). Flags functions exceeding the configured threshold (default is `10`).
* **False-Positive Considerations**: Highly branchy dispatch functions or lookup helpers (like matchers or routers) might have high complexity by design.
* **Remediation**: Refactor the function by extracting nested branches or cohesive logic blocks into smaller, single-purpose helper functions.

---

### MS002

* **Name**: Public Transfer Detection
* **Category**: `Security`
* **Default Severity**: `High`
* **CWE**: `CWE-284` (Improper Access Control)
* **Description**: Detects unrestricted transfer patterns where assets can be sent bypassing module-defined rules.
* **Detection Logic**: Scans function bodies and call lists for standard public transfer functions (e.g., `transfer::public_transfer` or `transfer::public_share_object`). Excludes the standard `init` module-level initialization function.
* **False-Positive Considerations**: Intentionally public-transferable assets (such as generic standard coins or open market NFTs) are acceptable.
* **Remediation**: Remove direct calls to unrestricted `public_transfer` for custom assets; instead, wrap transfer logic in module-defined, gated transfer routines.

---

### MS003

* **Name**: Capability Leakage
* **Category**: `Security`
* **Default Severity**: `High`
* **CWE**: `CWE-285` (Improper Authorization)
* **Description**: Detects when privileged capability objects (e.g. `AdminCap` or similar structs ending in `Cap`) are transferred or exposed.
* **Detection Logic**: Identifies functions having parameters containing `Cap` or `Capability`, with `public` or `entry` visibility, that perform standard object transfers on that parameter.
* **False-Positive Considerations**: System administration setup routines that explicitly distribute capabilities to newly authorized operators.
* **Remediation**: Ensure that capabilities are never transferred through general public pathways. Keep capabilities inside trusted administrative ownership.

---

### MS004

* **Name**: Shared Object Misuse
* **Category**: `Security`
* **Default Severity**: `High`
* **CWE**: `CWE-284` (Improper Access Control)
* **Description**: Detects improper mutation of shared objects without proper access control.
* **Detection Logic**: Flags `public` or `entry` functions that accept a mutable shared object parameter (e.g. `&mut State` or `&mut Store`) but have no accompanying Capability/Admin validation parameter in their signature.
* **False-Positive Considerations**: Shared state meant to be freely modifiable by anyone (such as open bulletin boards or public token pools).
* **Remediation**: Add a Capability validation parameter (such as `&AdminCap`) to authorize callers prior to mutating the shared object.

---

### MS005

* **Name**: Missing Access Control
* **Category**: `Security`
* **Default Severity**: `Critical`
* **CWE**: `CWE-285` (Improper Authorization)
* **Description**: Detects public entry functions that perform highly sensitive operations without proper authorization.
* **Detection Logic**: Scans `public`/`entry` functions whose name or body calls sensitive operations (like `mint`, `burn`, `withdraw`, `upgrade`, `pause`) but do not accept any Capability (`Cap`/`Capability`) or transaction context validation parameter.
* **False-Positive Considerations**: Highly public functions that intentionally allow anyone to mint free test tokens or withdraw their own deposits.
* **Remediation**: Gated sensitive functions with a required administrative Capability parameter or add robust sender validation inside the body.

---

### MS006

* **Name**: Timestamp / Clock Misuse
* **Category**: `Security`
* **Default Severity**: `Medium`
* **CWE**: `CWE-829` (Inclusion of Functionality from Untrusted Control Sphere)
* **Description**: Detects incorrect timing assumptions using unreliable epoch timestamps.
* **Detection Logic**: Scans functions that call or use `epoch_time` / `epoch_timestamp_ms` but do not accept the official `&sui::clock::Clock` object in their parameter signature.
* **False-Positive Considerations**: Coarse epoch-based timing checks that do not require millisecond precision.
* **Remediation**: Require `&sui::clock::Clock` in the function parameters and read the precise timestamp using `sui::clock::timestamp_ms(clock)`.

---

### MS007

* **Name**: Dynamic Field Risks
* **Category**: `Security`
* **Default Severity**: `Medium`
* **CWE**: `CWE-252` (Unchecked Return Value / Resource)
* **Description**: Detects potentially unsafe dynamic field additions that could lead to unexpected runtime transaction aborts.
* **Detection Logic**: Identifies calls to `dynamic_field::add` or `dynamic_object_field::add` that lack preceding checks (e.g. `exists_` or `has_field`) or capability restrictions.
* **False-Positive Considerations**: Safe execution flows where the field name is guaranteed to be unique or statically mapped.
* **Remediation**: Add defensive existence checks using `dynamic_field::exists_` before adding a field.

---

### MS008

* **Name**: Upgrade Policy Issues
* **Category**: `Security`
* **Default Severity**: `High`
* **CWE**: `CWE-285` (Improper Authorization)
* **Description**: Detects missing or weak package upgrade restrictions.
* **Detection Logic**: Scans upgrade/publishing functions with public or entry visibility that completely lack a required `UpgradeCap` parameter.
* **False-Positive Considerations**: Explicitly open/immutable packages or third-party wrappers.
* **Remediation**: Restrict custom upgrade routines to accept and validate the official `sui::package::UpgradeCap`.

---

### MS009

* **Name**: Treasury Misconfiguration
* **Category**: `Security`
* **Default Severity**: `High`
* **CWE**: `CWE-284` (Improper Access Control)
* **Description**: Detects insecure treasury management patterns where token supply controls could be leaked.
* **Detection Logic**: Flags public/entry functions accepting `TreasuryCap` that perform general object transfers or make it accessible.
* **False-Positive Considerations**: Initial token mint and lock routines that transfer the treasury cap to a governance multisig.
* **Remediation**: Avoid exposing `TreasuryCap` transfers in public functions. Limit interaction with treasury controls to dedicated admin-owned modules.

---

### MS010

* **Name**: Object Ownership Violations
* **Category**: `Security`
* **Default Severity**: `Medium`
* **CWE**: `CWE-284` (Improper Access Control)
* **Description**: Detects object nesting violations that obscure asset ownership and ID references.
* **Detection Logic**: Flags struct definitions (possessing the `key` ability) that directly contain/wrap another struct with the `key` ability.
* **False-Positive Considerations**: Standard, intended static wrapper structs.
* **Remediation**: Use ID-based referencing or dynamic object fields to model parent-child relationships instead of direct nesting.

---

### MS011

* **Name**: Randomness Misuse
* **Category**: `Security`
* **Default Severity**: `High`
* **CWE**: `CWE-337` (Predictable Seed in PRNG)
* **Description**: Detects usage of insecure, predictable values (like transaction digests or timestamps) for games/lotteries.
* **Detection Logic**: Scans functions with lottery/gaming-related names or logic that pull unpredictable seeds from predictable parameters like `tx_context::digest` or `epoch_time`.
* **False-Positive Considerations**: Non-critical applications where predictability does not present a economic or security risk.
* **Remediation**: Use Sui's official `sui::random` module or verifiable off-chain VRF endpoints.

---

### MS012

* **Name**: Gas Inefficiencies
* **Category**: `Performance`
* **Default Severity**: `Informational`
* **CWE**: `CWE-400` (Uncontrolled Resource Consumption)
* **Description**: Identifies expensive loop operations that could lead to out-of-gas aborts on-chain.
* **Detection Logic**: Scans public/entry functions containing loops (`while`, `loop`) that execute dynamic vector additions/deletions (such as `vector::push_back`).
* **False-Positive Considerations**: Small, statically-bounded loops where gas growth is negligible.
* **Remediation**: Pre-allocate vector sizes, strictly bound loop parameters, or handle large element vectors off-chain.

---

### MS013

* **Name**: Unused Capabilities
* **Category**: `Architecture`
* **Default Severity**: `Medium`
* **CWE**: `CWE-561` (Dead Code)
* **Description**: Detects capabilities defined in a module but never used to gate any operations.
* **Detection Logic**: Flags custom capability structs defined in a module (ending in `Cap` or `Capability`) that are never listed in the parameters of any function in that module.
* **False-Positive Considerations**: Capabilities defined in a module for the explicit use of external, integrating modules.
* **Remediation**: Delete unused capability definitions or incorporate them as parameters to secure privileged operations.

---

### MS014

* **Name**: Dead Public Functions
* **Category**: `Architecture`
* **Default Severity**: `Medium`
* **CWE**: `CWE-561` (Dead Code)
* **Description**: Detects public functions defined in application modules that are never called.
* **Detection Logic**: Identifies defined public functions with zero incoming callers in the call graph (excluding `entry` functions and standard `init`).
* **False-Positive Considerations**: Public functions inside library packages designed to be integrated by downstream developers.
* **Remediation**: Reduce function visibility to private/friend if it is internal, or remove dead helper logic.

---

### MS015

* **Name**: Duplicate Objects
* **Category**: `Architecture`
* **Default Severity**: `Medium`
* **CWE**: None
* **Description**: Detects duplicate object creations within a single function block.
* **Detection Logic**: Scans function bodies for multiple calls to standard object creation methods (e.g. `object::new`).
* **False-Positive Considerations**: Intentional batch-minting functions that initialize multiple objects concurrently.
* **Remediation**: Consolidate object instantiation routines or clearly document and separate individual object scopes.
