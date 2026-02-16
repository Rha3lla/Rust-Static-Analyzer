# Rule Catalog

This document defines the planned and implemented static analysis rules
for the Rust Static Analyzer project.
This is a work in progress and what is here will likely change frequenty

Each rule includes:
Rule ID (stable, like HA-RUST-0001)
Title
Default severity
Profile(s) (embedded, high_assurance, security, misra_rust)
Applies to (safe, unsafe, FFI, build)
CWE mapping (if any)
Status (planned, implemented, experimental)
One-line rationale


# Rule Catalog

This document defines the initial set of high-value static analysis rules
for the **Rust Static Analyzer** project.

Each rule has a **stable ID**, a default **severity**, applicable **profiles**,
and **security/safety mappings** where relevant.  
Rule IDs are immutable once published.

---

## Rule ID Scheme

- Prefix: `HA-RUST`
- Numeric range: `0001+`
- IDs are **stable and never reused**
- Gaps are allowed

Example: `HA-RUST-0007`

---

## Severity Levels

| Level | Meaning |
|-----|--------|
| Note | Informational, best practice |
| Warning | Risky or non-ideal |
| Error | Policy or safety violation |

---

## Profiles

- `embedded` – Bare-metal and resource-constrained systems
- `high_assurance` – Safety-critical / mission-critical code
- `security` – Secure coding and vulnerability prevention
- `misra_rust` – MISRA applicability and safety discipline for Rust

---

## Initial Rule Catalog (v0.1)

| Rule ID | Title | Severity | Profiles | Applies To | CWE | MISRA Applicability (Rust / Safe Rust) | MISRA Rust Category | Safety-Critical Rust Guideline Area | Status |
|-------|-------|----------|----------|------------|-----|---------------------------------------|---------------------|-------------------------------------|--------|
| HA-RUST-0001 | Unjustified `unsafe` block | Error | high_assurance, security, misra_rust | unsafe | CWE-119 | Yes / No | Required | Unsafety | Planned |
| HA-RUST-0002 | `unsafe` outside approved modules | Error | high_assurance, misra_rust | unsafe | CWE-119 | Yes / No | Required | Unsafety | Planned |
| HA-RUST-0003 | Missing `SAFETY:` comment in unsafe block | Error | high_assurance | unsafe | CWE-119 | Yes / No | Required | Unsafety | Planned |
| HA-RUST-0004 | Use of `mem::transmute` | Error | high_assurance, security, misra_rust | unsafe | CWE-681 | Yes / No | Required | Unsafety | Planned |
| HA-RUST-0005 | Raw pointer dereference | Error | high_assurance, security | unsafe | CWE-119 | Yes / No | Required | Unsafety | Planned |
| HA-RUST-0006 | Unchecked narrowing integer cast | Warning | high_assurance, embedded | safe / unsafe | CWE-190 | Yes / Yes | Advisory | Values & Expressions | Planned |
| HA-RUST-0007 | Arithmetic overflow not explicitly handled | Warning | high_assurance, embedded | safe | CWE-190 | Yes / Yes | Advisory | Values & Expressions | Planned |
| HA-RUST-0008 | `unwrap()` or `expect()` outside tests | Error | high_assurance, embedded, security | safe | CWE-703 | Yes / Yes | Required | Exceptions & Errors | Planned |
| HA-RUST-0009 | Panic possible in production build | Error | embedded, high_assurance | build | CWE-248 | Yes / Yes | Required | Program Structure & Compilation | Planned |
| HA-RUST-0010 | Dynamic allocation in restricted context | Error | embedded | safe | CWE-770 | Yes / Yes | Required | Program Structure & Compilation | Planned |
| HA-RUST-0011 | Unbounded loop without explicit exit | Warning | embedded, high_assurance | safe | CWE-835 | Yes / Yes | Advisory | Statements & Functions | Planned |
| HA-RUST-0012 | Recursion in embedded profile | Warning | embedded | safe | CWE-674 | Yes / Yes | Advisory | Functions | Planned |
| HA-RUST-0013 | Indexing without checked abstraction | Warning | high_assurance, security | safe | CWE-125 | Yes / Yes | Advisory | Values & Expressions | Planned |
| HA-RUST-0014 | FFI function without safe wrapper | Error | security, misra_rust | FFI | CWE-676 | Yes / No | Required | FFI | Planned |
| HA-RUST-0015 | `extern "C"` missing ABI assumptions | Warning | security, misra_rust | FFI | CWE-686 | Yes / No | Required | FFI | Planned |
| HA-RUST-0016 | Use of `static mut` | Error | high_assurance, misra_rust | unsafe | CWE-362 | Yes / No | Required | Concurrency & Unsafety | Planned |
| HA-RUST-0017 | Atomic operations lack ordering rationale | Warning | high_assurance, embedded | safe | CWE-662 | Yes / Yes | Advisory | Concurrency | Planned |
| HA-RUST-0018 | Interior mutability (`Cell`, `RefCell`) | Warning | high_assurance | safe | CWE-362 | Yes / Yes | Advisory | Ownership & Concurrency | Planned |
| HA-RUST-0019 | Floating-point use in deterministic profile | Warning | embedded, high_assurance | safe | CWE-682 | Yes / Yes | Advisory | Values | Planned |
| HA-RUST-0020 | Unknown or unstable target configuration | Warning | embedded | build | — | Yes / Yes | Advisory | Program Structure & Compilation | Planned |
| HA-RUST-0021 | Dependency with RustSec advisory | Error | security | dependency | CWE-1104 | Yes / Yes | Required | Supply Chain | Planned |
| HA-RUST-0022 | Deprecated or yanked dependency | Warning | security | dependency | CWE-1104 | Yes / Yes | Advisory | Supply Chain | Planned |
| HA-RUST-0023 | Error type erased into `Box<dyn Error>` | Warning | high_assurance | safe | CWE-391 | Yes / Yes | Advisory | Exceptions & Errors | Planned |
| HA-RUST-0024 | Implicit panic via indexing or slicing | Warning | high_assurance, embedded | safe | CWE-125 | Yes / Yes | Advisory | Exceptions & Errors | Planned |
| HA-RUST-0025 | Public unsafe API missing documentation | Error | high_assurance, misra_rust | unsafe | CWE-749 | Yes / No | Required | Unsafety | Planned |




---

## Notes

- CWE mappings are **best-fit**, not claims of vulnerability.
- Some rules may be configurable per project or target.
- Suppressions should require explicit justification.

---

## Status

All rules listed here are **planned** unless otherwise noted.
Implementation order prioritizes:
1. Decidable rules
2. Embedded safety impact
3. CI/CD usefulness
