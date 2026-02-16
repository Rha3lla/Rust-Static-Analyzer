# HA-RUST-0001 — Unjustified `unsafe` Block

## Rule Summary

**ID:** HA-RUST-0001  
**Title:** Unjustified `unsafe` block  
**Severity:** Error  
**Profiles:** high_assurance, security, misra_rust  
**Scope:** unsafe  

This rule flags any `unsafe` construct that does not have an associated,
explicit safety justification explaining the invariants relied upon.

---

## Description

Rust’s `unsafe` keyword permits operations that the compiler cannot
statically verify for memory safety, aliasing, or concurrency correctness.

In high-assurance, embedded, and security-critical systems, the *absence*
of a documented safety argument for unsafe code represents an unacceptable
risk.

This rule enforces that **every unsafe construct must be explicitly justified**
with a human-readable explanation of *why the code is sound*.

---

## Rationale

Requiring explicit safety justification:

- makes safety invariants visible and reviewable
- preserves intent across refactoring and maintenance
- enables informed peer review and audit
- aligns with MISRA applicability principles
- enforces Rust safety-critical best practices

An undocumented unsafe block is treated as a defect, regardless of whether
the code “works.”

---

## Detection Criteria

This rule triggers when:

- an unsafe construct is present:
  - `unsafe { ... }`
  - `unsafe fn`
  - `unsafe impl`
  - `unsafe trait`
- **and**
- no associated safety justification is found according to the
  **Safety Comment Association Policy**

---

## Safety Justification Requirements

### Required Token

A safety justification **must** include the case-sensitive token:



SAFETY:


### Association Rules

Association between a `SAFETY:` comment and an unsafe construct is governed
by the shared policy document:

➡️ **[Safety Comment Association Policy](../policies/safety-comment-association.md)**

This policy defines:
- valid adjacency rules
- attribute handling
- macro-related behavior
- nested unsafe requirements
- content quality expectations

---

## Examples

### ❌ Non-compliant

```rust
unsafe {
    core::ptr::copy_nonoverlapping(src, dst, len);
}



## Compliant
// SAFETY:
// - `src` and `dst` are valid for `len` elements
// - regions do not overlap
// - lifetime is guaranteed by the caller
unsafe {
    core::ptr::copy_nonoverlapping(src, dst, len);
}



CWE Mapping

CWE-119 — Improper Restriction of Operations within the Bounds of a Memory Buffer

This mapping reflects the class of failures that undocumented or
misunderstood unsafe code can introduce.

MISRA Applicability
Rust (general): Applicable
Safe Rust: Not applicable (rule concerns unsafe)
MISRA Rust Category: Required

This rule aligns with MISRA’s intent to:
restrict unsafe constructs and require explicit justification for deviations from safe subsets


## Safety-Critical Rust Guideline Mapping

Area: Unsafety
Guideline: Unsafe code must be minimal, isolated, and documented

This rule directly enforces the documentation requirement.

## Deviation and Suppression
Allowed Deviations

Deviation from this rule is permitted only if:

  A safety justification is provided elsewhere (e.g., module-level)

  The justification explicitly references the unsafe block

  The rationale is reviewable and auditable

Suppression Requirements
Any suppression must:
  reference HA-RUST-0001
  explain why local justification is not feasible
  identify where the safety argument is documented

Silent suppression is not permitted.

Implementation Notes
  Detection operates on the Rust HIR
  Comment association is heuristic but conservative
  False negatives are preferred over false positives

Evidence Expectations
To validate this rule:
  Positive test: undocumented unsafe block is flagged
  Negative test: documented unsafe block is accepted
  Edge case tests:
    attribute sandwich
    nested unsafe
    macro-origin unsafe

Related Rules

HA-RUST-0002 — unsafe outside approved modules

HA-RUST-0003 — Missing SAFETY: comment in unsafe block

HA-RUST-0025 — Public unsafe API missing documentation
