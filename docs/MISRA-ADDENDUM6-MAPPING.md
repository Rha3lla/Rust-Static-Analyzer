# MISRA-C:2025 Addendum 6 — Rust Applicability Mapping

This document tracks the applicability and implementation status of MISRA-C:2025 guidelines as they apply to Rust, per Addendum 6.

## Document Info

| Field | Value |
|-------|-------|
| MISRA-C Version | 2025 |
| Addendum | 6 (Applicability to Rust) |
| Last Updated | 2025-01-XX |
| Tool | HAAT (High-Assurance Analysis Tool) |

---

## Legend

### Rust Applicability Status

| Status | Meaning |
|--------|---------|
| ✅ Language | Rust compiler/language enforces this by design |
| 🔧 Tool | Requires static analysis tooling (HAAT implements) |
| 📖 Review | Requires manual code review |
| ⚠️ Partial | Partially enforced; some aspects need tooling |
| ➖ N/A | Not applicable to Rust |

### Obligation (per MISRA)

| Level | Meaning |
|-------|---------|
| Mandatory | Must be followed without exception |
| Required | Must be followed except with documented deviation |
| Advisory | Recommended best practice |

### HAAT Severity Assignment

| Severity | Criteria |
|----------|----------|
| Critical | Undefined behavior, memory safety violation, exploitable vulnerability |
| High | Likely security/safety impact, data integrity risk |
| Medium | Potential issue under specific conditions |
| Low | Code quality, maintainability, defense-in-depth |

### Implementation Status

| Status | Meaning |
|--------|---------|
| ⬜ Backlog | Not yet planned |
| 🚧 Planned | In roadmap |
| 🔨 In Progress | Currently implementing |
| ✔️ Done | Implemented and tested |
| ➖ Skip | Will not implement (language-enforced or N/A) |

---

## Directives

| ID | Title | C Status | Rust General | Safe Rust | Severity | HAAT Status | Notes |
|----|-------|----------|--------------|-----------|----------|-------------|-------|
| | | | | | | | |

---

## Rules

| ID | Title | C Status | Rust General | Safe Rust | Severity | HAAT Status | Notes |
|----|-------|----------|--------------|-----------|----------|-------------|-------|
| | | | | | | | |
