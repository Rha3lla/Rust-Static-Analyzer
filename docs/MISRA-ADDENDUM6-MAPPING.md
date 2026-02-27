# MISRA-C:2025 Addendum 6 — Applicability to Rust

Tracking document for rule implementation status.

## Legend

| Status | Meaning |
|--------|---------|
| ✅ Enforced | Rust language/compiler guarantees compliance |
| 🔧 Tool | Requires static analysis tooling |
| 📖 Review | Requires manual code review |
| ➖ N/A | Not applicable to Rust |
| 🚧 Planned | Implementation planned |
| ✔️ Done | Implemented in HAAT |

## Severity Mapping Rationale

| Severity | Criteria |
|----------|----------|
| Critical | Undefined behavior, memory safety violation, guaranteed exploit vector |
| High | Likely security/safety impact, data integrity risk |
| Medium | Potential issue under specific conditions, defense-in-depth |
| Low | Code quality, maintainability, best practice |

---

## Directives

| ID | Title | Obligation | Rust Status | Severity | HAAT Status |
|----|-------|------------|-------------|----------|-------------|
| Dir 1.1 | Implementation-defined behavior | Required | 🔧 Tool | Medium | 🚧 Planned |
| Dir 2.1 | Assembly language | Required | 🔧 Tool | High | 🚧 Planned |
| Dir 3.1 | Code comprehension | Required | 📖 Review | Low | ➖ N/A |
| Dir 4.1 | Run-time failures | Required | 🔧 Tool | Critical | 🚧 Planned |
| ... | ... | ... | ... | ... | ... |

---

## Rules

| ID | Title | Obligation | Rust Status | Severity | HAAT Status |
|----|-------|------------|-------------|----------|-------------|
| Rule 1.1 | Non-conforming code | Required | 🔧 Tool | High | 🚧 Planned |
| Rule 1.2 | Undefined behavior | Required | ✅ Enforced | Critical | ✅ Enforced |
| Rule 1.3 | Unspecified behavior | Required | 🔧 Tool | High | 🚧 Planned |
| ... | ... | ... | ... | ... | ... |

---

## Notes

- Rules marked "✅ Enforced" are compliance claims based on Rust's language guarantees
- These claims should be documented in certification evidence packages
- Tool-enforced rules are the implementation priority for HAAT
