# Traceability and Compliance Model

This document defines how the **Rust Static Analyzer** establishes and maintains
traceability between analysis rules, external standards, and implementation evidence.

The goal is to support **high-assurance**, **embedded**, and **security-critical**
Rust development by providing transparent, auditable links between:
- tool rules
- recognized safety and security taxonomies
- documented rationale and evidence

This tool does **not** claim certification or formal compliance on its own.
Instead, it provides structured evidence to support downstream assurance activities.

---

## 1. What Traceability Means in This Project

For each static analysis rule, traceability means:

1. The rule has a **stable, immutable identifier**
2. The rule is mapped to **external references** where applicable
3. The rule has a **documented rationale**
4. The rule has **clear applicability boundaries**
5. Deviations are **explicit, justified, and reviewable**
6. Tool output can be traced back to:
   - the rule definition
   - the source location
   - the external guidance it enforces

---

## 2. Stable Rule Identification

Each rule is assigned a stable identifier of the form:

HA-RUST-XXXX


Rules:
- IDs are never reused
- IDs never change once published
- Gaps in numbering are allowed
- IDs appear consistently in:
  - documentation
  - tool output
  - SARIF reports
  - CI logs

This ensures long-term traceability across tool versions.

---

## 3. External Reference Frameworks

### 3.1 CWE (Common Weakness Enumeration)

Where applicable, rules are mapped to one or more **CWE IDs** maintained by MITRE.

- CWE mappings indicate **conceptual alignment**, not proof of vulnerability
- A single rule may map to multiple CWEs
- Not all rules require a CWE mapping (e.g., build or process rules)

CWE is used as a **security taxonomy**, not a vulnerability database.

---

### 3.2 MISRA Applicability to Rust

This project aligns with the **MISRA C:2025 Addendum 6 – Applicability to Rust**.

Key principles:
- MISRA guidance is treated as **applicability**, not direct compliance
- Applicability is classified separately for:
  - *Rust (general)*: includes unsafe code and FFI
  - *Safe Rust*: excludes `unsafe` and `extern`
- Rules are categorized as:
  - Required
  - Advisory
  - Disapplied / Not Applicable (where appropriate)

The tool does **not** claim MISRA certification.
Instead, it enforces constraints that are consistent with MISRA’s intent
where MISRA deems them applicable to Rust.

---

### 3.3 Safety-Critical Rust Coding Guidelines

Rules are also mapped to sections of the
**Safety-Critical Rust Coding Guidelines**
maintained by the Rust community.

These guidelines provide:
- Rust-native safety rationale
- Guidance for unsafe code, FFI, error handling, concurrency, and structure
- A deviation-based compliance model compatible with high-assurance development

The analyzer treats these guidelines as a **primary Rust-specific safety reference**.

---

## 4. Rule Metadata and Traceability Fields

Each rule definition includes, at minimum:

- Rule ID
- Title
- Severity
- Applicable profiles
- Scope (`safe`, `unsafe`, `ffi`, `build`, `dependency`)
- CWE mappings (if applicable)
- MISRA applicability (Rust / Safe Rust)
- MISRA Rust category (Required / Advisory)
- Safety-Critical Rust guideline area
- Rationale
- Examples (positive and negative)
- Deviation / suppression guidance

This metadata is reflected in both documentation and machine-readable output.

---

## 5. Deviation and Suppression Model

### 5.1 Principle

In high-assurance systems, deviations are sometimes necessary — but must be:
- explicit
- justified
- reviewable

Silent suppression is not permitted.

---

### 5.2 Suppression Requirements

Any suppression of a rule must include:
- the rule ID
- a justification explaining why the deviation is safe
- scope limited to the smallest possible region

Suppressions may be expressed via:
- structured comments
- attributes (future)
- configuration files (future)

The exact syntax is defined by the tool, but justification is mandatory.

---

## 6. Evidence and Verification

Each rule should be supported by evidence in the repository, including:
- positive test cases (rule triggers correctly)
- negative test cases (rule does not over-trigger)
- documentation explaining intent and limits

This evidence supports:
- internal review
- CI enforcement
- future qualification or certification activities

---

## 7. Tool Output and SARIF

The analyzer is designed to emit results in **SARIF** format.

Each reported finding includes:
- the stable rule ID
- source location
- severity
- rule metadata
- links to rule documentation

This enables:
- CI/CD integration
- result baselining
- long-term audit traceability

---

## 8. Scope and Limitations

This tool:
- supports safety and security engineering
- does not replace formal verification
- does not certify compliance by itself

Final responsibility for system safety and compliance
remains with the system developer and assessor.

---

## 9. Evolution of Traceability

This document will evolve as:
- additional rules are added
- MISRA applicability mappings are refined
- new external guidance emerges

Changes will preserve backward traceability for all existing rule IDs.
