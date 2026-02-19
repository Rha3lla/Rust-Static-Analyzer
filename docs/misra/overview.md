This tool does not claim MISRA certification.
It provides automated enforcement and reporting for MISRA guidelines that are applicable to Rust, based on published MISRA applicability guidance.
Final MISRA compliance determination remains the responsibility of the development organization.

Some MISRA guidelines are process-oriented and out of scope for static analysis
Those will be explicitly documented as “Not Enforceable by Static Analysis”

Regarding MISRA, the goal of MISRA compliance output means:
--Rule-level compliance results
--MISRA category summaries (Required vs Advisory)
--Deviation listings
--Coverage statements (what is / is not enforced)

But not:
--a binary “MISRA compliant / non-compliant” verdict
That verdict belongs to the system owner.

# MISRA Applicability and Compliance Support

This document describes how the Rust Static Analyzer provides support for enforcing and reporting on MISRA guidelines that are applicable to Rust.

The intent of this document is to define the scope, limitations, and interpretation of MISRA-related analysis performed by this tool, and to establish a clear and defensible compliance model suitable for high-assurance and embedded development workflows.


## Scope and Non-Certification Statement

This tool does **not** claim MISRA certification.

The Rust Static Analyzer provides automated enforcement and reporting for MISRA guidelines that are applicable to Rust, based on publicly available MISRA applicability guidance. The tool is intended to assist development teams in identifying potential violations, supporting deviation management, and producing traceable evidence for compliance activities.

Final determination of MISRA compliance, including acceptance of deviations and assessment of process-oriented guidelines, remains the responsibility of the development organization.


## MISRA Applicability Model

MISRA guidelines were originally developed for the C language family. As such, not all guidelines are directly applicable to Rust. This tool adopts an applicability-based approach rather than attempting blanket enforcement.

The analyzer distinguishes between two primary applicability modes:

### Rust General Applicability

This mode applies to Rust programs that may use low-level language features, including but not limited to:

- `unsafe` blocks and functions
- Foreign Function Interface (FFI) boundaries
- Low-level memory manipulation
- Concurrency primitives requiring explicit ordering guarantees

This mode is intended for:
- Embedded systems
- Systems programming
- Mixed-criticality codebases

### Safe Rust Subset Applicability

This mode applies to code written entirely within the Safe Rust subset, excluding explicit use of `unsafe`.

This mode is intended for:
- High-assurance application logic
- Safety-critical modules that prohibit unsafe code
- Environments where stricter language constraints are desired


## Rule Mapping and Traceability

MISRA enforcement within the tool is performed through explicit mapping between internal analysis rules and MISRA guidelines.

The following principles apply:

- Each internal rule may map to one or more MISRA guidelines.
- Some MISRA guidelines may be partially enforced by multiple internal rules.
- Rules that do not meaningfully correspond to a MISRA guideline are explicitly marked as *Not Applicable*.
- No implicit or inferred mappings are performed.

Mapping information is maintained as a first-class artifact and includes:

- Internal rule identifier
- MISRA guideline reference
- Applicability classification (Rust general or Safe Rust)
- MISRA category (Required or Advisory)



## Guidelines Outside Static Analysis Scope

Some MISRA guidelines are process-oriented or organizational in nature and cannot be fully enforced through static code analysis.

Examples include:
- Development process requirements
- Documentation or review practices
- System-level architectural constraints

Such guidelines are explicitly documented as outside the scope of static enforcement and are not reported as violations by the tool.



## Deviations and Justifications

The tool supports explicit deviation handling in accordance with MISRA expectations.

When a MISRA-mapped rule is violated, the tool allows deviations to be recorded with documented justification. Deviations may be expressed through:

- Inline code annotations
- Configuration-based suppression policies

Each deviation is expected to include:
- The internal rule identifier
- A human-readable justification

An audit mode is provided to disable suppressions and report all findings, enabling independent review of deviations and justifications.



## Compliance Reporting Outputs

The analyzer produces structured outputs intended to support MISRA compliance activities, including:

- Rule-level findings
- Severity classification
- MISRA category summaries (Required vs Advisory)
- Deviation listings and justifications
- Coverage statements describing which guidelines are enforced

The tool does not produce a binary declaration of MISRA compliance or non-compliance. Compliance determinations remain the responsibility of the system owner.



## Versioning and Evolution

MISRA applicability mappings and enforcement behavior may evolve over time as the tool matures.

All changes affecting MISRA mapping, applicability, or reporting behavior are versioned and documented to support traceability and audit requirements.
