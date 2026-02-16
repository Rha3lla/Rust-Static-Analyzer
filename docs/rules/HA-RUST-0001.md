# HA-RUST-0001 — Unjustified `unsafe` Block

## Rule Summary

**ID:** HA-RUST-0001  
**Title:** Unjustified `unsafe` block  
**Severity:** Error  
**Profiles:** high_assurance, security, misra_rust  
**Scope:** unsafe  

This rule flags any `unsafe` block that does not include an explicit,
human-readable justification describing the safety invariants relied upon.

---

## Description

Rust’s `unsafe` keyword permits operations that the compiler cannot
statically verify for memory safety, aliasing, or concurrency correctness.

While `unsafe` is a necessary and powerful escape hatch, its misuse or
under-documentation represents a significant risk in high-assurance,
embedded, and security-critical systems.

This rule enforces that **every `unsafe` block must be accompanied by a
clear justification** explaining *why the code is sound*.

---

## Rationale

In safety-critical and security-sensitive Rust codebases:

- The *absence* of a justification is itself a defect
- Safety invariants must be preserved across refactors and maintenance
- Reviewers and auditors must be able to assess correctness without
  reverse-engineering intent

Requiring an explicit justification:
- documents assumptions
- enables informed review
- aligns with both MISRA intent and Rust safety-critical guidance
- supports long-term traceability

---

## Detection Criteria

This rule triggers when:

- An `unsafe { ... }` block is present
- AND no immediately associated safety justification is found

A justification is considered associated if it:
- precedes the `unsafe` block
- is adjacent (no intervening non-comment code)
- clearly refers to the unsafe operation

---

## Required Justification Format

The justification **must** include the keyword:

SAFETY:

and explain the safety invariants being upheld.

### Acceptable Example

```rust
// SAFETY:
// - `ptr` is guaranteed to be non-null
// - `ptr` points to a valid object of type `T`
// - lifetime is bounded by the caller
unsafe {
    *ptr = value;
}

### Unacceptable Example
// This is safe because we know it works
unsafe {
    *ptr = value;
}


### Examples
## Non-Compliant
unsafe {
    core::ptr::copy_nonoverlapping(src, dst, len);
}


## Compliant
// SAFETY:
// - `src` and `dst` are valid for `len` elements
// - regions do not overlap
// - caller guarantees lifetime validity
unsafe {
    core::ptr::copy_nonoverlapping(src, dst, len);


---

## Why this works as a template
- Clear **rule metadata up top**
- Explicit **rationale** (auditors love this)
- Concrete **detection criteria**
- Strong **examples**
- Traceability sections separated cleanly
- Deviation policy spelled out (MISRA-aligned)
- Implementation notes guide *future you*


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

restrict unsafe constructs

require explicit justification for deviations from safe subsets

Safety-Critical Rust Guideline Mapping

Area: Unsafety

Guideline: Unsafe code must be minimal, isolated, and documented

This rule directly enforces the documentation requirement.

Deviation and Suppression
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

Edge case tests: nested unsafe blocks, macros, cfg-gated code

Related Rules

HA-RUST-0002 — unsafe outside approved modules

HA-RUST-0003 — Missing SAFETY: comment in unsafe block

HA-RUST-0025 — Public unsafe API missing documentation
