# Rust Static Analyzer

**A rules-driven static analysis tool for Rust — designed for high-assurance systems, embedded development, and CI/CD integration.**  
This project aims to fill the gap between compiler-level checks and the rigorous demands of safety, security, and embedded coding standards.

---

## 📌 Why This Tool Exists

Rust today benefits from a strong type system and ownership model, but:
- The built-in tools (`rustc`, Clippy) focus on correctness, style, or ergonomic warnings — *not high-assurance policy enforcement*.
- There is no first-class Rust analyzer tailored for **embedded constraints**, **safety/mission-critical environments**, or **security policy alignment**.
- Teams building on bare-metal and cross-compiling for diverse targets need a **ruleset engine** that works equally well on Linux and Windows, integrates seamlessly into pipelines, and adapts to project-specific policies.

This analyzer is being built to solve that need.

---

## 🧠 Vision

This project provides:
- A **configurable static analysis engine** for Rust source code.
- **Profiles** tailored to:
  - Embedded systems
  - High-assurance coding
  - Security/CWE alignment
  - MISRA-applicability for Rust
- A focus on:
  - Decidability
  - Audit-grade metadata
  - SARIF reporting for CI/CD
  - Ease of integration for existing Rust teams

---

## 🚀 Features

- **High-assurance rule enforcement**  
  Rule definitions with rationale, severity, and traceable documentation.

- **Security advisory scanning**  
  Integrates with the RustSec ecosystem to flag known crate vulnerabilities.

- **Multi-profile support**  
  Switch between enforcement profiles or customize rules per project.

- **SARIF output**  
  Designed to plug into GitHub/GitLab/Bitbucket code scanning and dashboards.

- **Cross-platform**  
  Runs on Windows and Linux, and is agnostic to chip architecture and OS presence.

- **CI/CD-ready**  
  Non-zero exit codes on policy violations, suppression support, and diff mode for incremental adoption.

---

## 🧩 Planned Rule Profiles

| Profile | Focus | Example Constraints |
|---------|-------|--------------------|
| `embedded` | Bare-metal / resource-constrained | No dynamic allocation in hot loops, no panics in production |
| `high_assurance` | Safety-critical systems | Ban unchecked casts, explicit error handling |
| `security` | Secure coding | Disallow unsafe FFI without wrappers, enforce bounds checks |
| `misra_rust` | MISRA applicability | Rust mappings of MISRA-C constraints based on MISRA Addendum 6 |

---

## 🛠️ Usage

> ⚠️ This tool is under active development. 



GOALS
1. Determine rulesets and requirements that need to go into the tool.
2. 2. Find any/all CWE/CVEs against Rust to incorporate into the tool.
3. Need to be platform agnostic so it can be run on both a Linux and a Windows environment.
4. Need to be chip agnostic so it can be run against bare-metal or OS-based implementations.
5. Ideally this would be something developers could implement as part of their CI/CD pipeline to run against code developed as part of regular testing
6. There should be rules that comply with MISRA coding standards as they apply to Rust
7. There will probably be more requirements that are discovered along the way. 


Rule Definition Format

Each rule includes:
Unique stable ID
Severity level
CWE mappings
MISRA applicability annotations
Rationale and examples
Suppression guidelines
This metadata supports transparency and auditability for regulated environments.

Acknowledgements
Inspired by limitations in existing tooling like rustc and Clippy
Leveraging the RustSec Advisory Database
Aligned to emerging Rust safety-critical practices
