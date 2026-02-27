# Architecture Overview

## Purpose

HAAT (High-Assurance Analysis Tool) is a static analysis tool for Rust, targeting DoD and military certification requirements.

## Design Principles

1. **Reproducibility** — Same input produces identical output across runs and platforms
2. **Traceability** — Every finding links to a specific rule and can be audited
3. **Extensibility** — New rules can be added without modifying core engine
4. **Platform Agnostic** — Runs on Windows and Linux development environments
5. **Certification-Ready** — Output formats support evidence package generation

## Component Architecture

### haat-core
Core types and traits shared across all components.
- Finding, Severity, SourceLocation types
- Rule trait definition
- No external dependencies beyond std (minimize supply chain risk)

### haat-rules-misra
MISRA-C:2025 Addendum 6 rule implementations.
- Each rule in its own module
- Rules depend only on haat-core
- Declarative rule metadata in TOML, logic in Rust

### haat-driver
Rust compiler integration layer.
- Interfaces with rustc for HIR/MIR access
- Provides analysis context to rules
- Handles multi-file project traversal

### haat-reporter
Output generation.
- SARIF 2.1.0 (primary, for tool integration)
- HTML (human-readable reports)
- Compliance matrix (certification evidence)

### haat-cli
Command-line interface.
- Argument parsing
- Configuration loading
- Human-friendly output

## Data Flow

