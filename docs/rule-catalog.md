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

## Initial Rule Set (v0.1)

| Rule ID | Title | Severity | Profiles | Applies To | CWE | Rationale |
|------|------|---------|---------|-----------|-----|----------|
| HA-RUST-0001 | Unjustified `unsafe` block | Error | high_assurance, security, misra_rust | unsafe | CWE-119 | Unsafe code must be minimal, explicit, and justified |
| HA-RUST-0002 | `unsafe` outside approved modules | Error | high_assurance, misra_rust | unsafe | CWE-119 | Constrains unsafe code to audited locations |
| HA-RUST-0003 | Missing `SAFETY:` comment in unsafe block | Error | high_assurance | unsafe | CWE-119 | Enforces human-readable safety invariants |
| HA-RUST-0004 | Use of `mem::transmute` | Error | high_assurance, security, misra_rust | unsafe | CWE-681 | Extremely error-prone and rarely justified |
| HA-RUST-0005 | Raw pointer dereference | Error | high_assurance, security | unsafe | CWE-119 | Pointer safety must be explicit and reviewed |
| HA-RUST-0006 | Unchecked integer cast (narrowing) | Warning | high_assurance, embedded | safe/unsafe | CWE-190 | Silent truncation risks correctness |
| HA-RUST-0007 | Arithmetic overflow not explicitly handled | Warning | high_assurance, embedded | safe | CWE-190 | Determinism and correctness requirement |
| HA-RUST-0008 | Use of `unwrap()` or `expect()` outside tests | Error | high_assurance, embedded, security | safe | CWE-703 | Panics are unacceptable in production systems |
| HA-RUST-0009 | Panic possible in production build | Error | embedded, high_assurance | build | CWE-248 | Embedded systems must not unwind unexpectedly |
| HA-RUST-0010 | Dynamic allocation in restricted context | Error | embedded | safe | CWE-770 | Allocation may be forbidden or bounded |
| HA-RUST-0011 | Unbounded loop without exit condition | Warning | embedded, high_assurance | safe | CWE-835 | Infinite loops must be explicit and justified |
| HA-RUST-0012 | Recursion used in embedded profile | Warning | embedded | safe | CWE-674 | Stack usage must be predictable |
| HA-RUST-0013 | Indexing without bounds check abstraction | Warning | high_assurance, security | safe | CWE-125 | Bounds errors can still occur logically |
| HA-RUST-0014 | FFI function without safe wrapper | Error | security, misra_rust | FFI | CWE-676 | Unsafe interfaces must be isolated |
| HA-RUST-0015 | `extern "C"` without documented ABI assumptions | Warning | security, misra_rust | FFI | CWE-686 | ABI mismatches are catastrophic |
| HA-RUST-0016 | Use of `static mut` | Error | high_assurance, misra_rust | unsafe | CWE-362 | Global mutable state is dangerous |
| HA-RUST-0017 | Atomic operations without ordering rationale | Warning | high_assurance, embedded | safe | CWE-662 | Memory ordering must be intentional |
| HA-RUST-0018 | Use of interior mutability (`RefCell`, `Cell`) | Warning | high_assurance | safe | CWE-362 | Can violate assumptions in safety-critical code |
| HA-RUST-0019 | Floating-point use in deterministic profile | Warning | embedded, high_assurance | safe | CWE-682 | FP determinism may not be guaranteed |
| HA-RUST-0020 | Build uses unknown or unstable target | Warning | embedded | build | — | Target assumptions must be explicit |
| HA-RUST-0021 | Dependency with known RustSec advisory | Error | security | dependency | CWE-1104 | Known vulnerable code must not ship |
| HA-RUST-0022 | Use of deprecated or yanked crate | Warning | security | dependency | CWE-1104 | Indicates maintenance or security risk |
| HA-RUST-0023 | Error type erased into `Box<dyn Error>` | Warning | high_assurance | safe | CWE-391 | Loss of error specificity |
| HA-RUST-0024 | Implicit panic via indexing or slicing | Warning | high_assurance, embedded | safe | CWE-125 | Panics must be intentional |
| HA-RUST-0025 | Missing documentation on public unsafe API | Error | high_assurance, misra_rust | unsafe | CWE-749 | Unsafe contracts must be documented |

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
