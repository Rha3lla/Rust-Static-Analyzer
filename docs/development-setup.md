# Development Setup (Windows)

This document describes the recommended local development environment for the
Rust Static Analyzer project on **Windows**.

The goal is to ensure contributors can:
- build the workspace reliably
- run tests consistently
- avoid common Windows toolchain pitfalls
- prepare for future rustc-driver integration (nightly)

---

## 1) Prerequisites

### GitHub Desktop
Recommended for cloning, committing, and pushing without CLI complexity.

Repo URL:
- https://github.com/Rha3lla/Rust-Static-Analyzer

### Rust Toolchain (rustup)
Install Rust via `rustup` (preferred) so toolchains and components are easy to manage.

Required:
- stable toolchain
- `cargo`, `rustc`

Recommended components:
- `rustfmt`
- `clippy`

Recommended additional toolchain:
- nightly (installed, but not default)

---

## 2) Visual Studio (MSVC) Requirements

This project targets the standard Rust-on-Windows toolchain:
- `x86_64-pc-windows-msvc`

You must have MSVC build tools and a Windows SDK installed.

### Visual Studio Installer
Open **Visual Studio Installer** → **Modify** your Visual Studio installation and ensure this workload is selected:

✅ **Desktop development with C++**

Confirm these components are installed (names may vary slightly):
- ✅ MSVC C++ x64/x86 build tools (v143 or current)
- ✅ Windows 10 or Windows 11 SDK

### Note on `where cl`
On many systems, `cl.exe` is not available in a normal PowerShell PATH, even when installed.
This is normal.

To verify MSVC tools exist, use:
- **“x64 Native Tools Command Prompt for VS”**
and run `cl`.

Rust typically discovers MSVC via Visual Studio configuration, not only PATH.

---

## 3) Recommended Local Folder Layout

To avoid Windows path-length issues, keep a short path such as:

`C:\dev\Rust-Static-Analyzer`

Avoid deeply nested directories (e.g., long OneDrive paths).

---

## 4) Cloning the Repository (GitHub Desktop)

1. Open **GitHub Desktop**
2. **File → Clone repository…**
3. Select **URL** and paste the repo URL
4. Choose a local path (e.g., `C:\dev\`)
5. Clone

---

## 5) IntelliJ Setup

This project works with IntelliJ + Rust plugin.

Recommended:
- open the repository root folder (the workspace root)
- allow IntelliJ to load the Cargo workspace when prompted

Workspace root should contain:
- `Cargo.toml`
- `crates/`
- `docs/`

---

## 6) Build and Run

From the repository root:

### Build
```powershell
cargo build

## Run prototype subcommand
cargo run -p cargo-ha-lint


##Expected output example
cargo ha-lint (prototype) 0.1.0


## Tests
cargo test


### 7. Toolchain Management
Install nightly (do not set as default)

Nightly will be needed later for compiler-internal analysis, but stable is preferred for early scaffolding.

Install:
rustup toolchain install nightly

Keep stable default
rustup default stable


### 8) Troubleshooting
Error: could not find Cargo.toml

You are not running commands from the workspace root.
Ensure your terminal current directory contains the root Cargo.toml.

## Linker / SDK errors

Common causes:
  Desktop development with C++ workload not installed
  Windows SDK not installed
  Visual Studio update incomplete

Fix:
Visual Studio Installer → Modify → ensure C++ workload + SDK are selected


### 9) Optional Tools (Later)

These are not required for early milestones, but may be useful later:

  cargo-audit (RustSec dependency vulnerability scanning)
  cargo-deny (dependency policies)
  cargo-nextest (faster test execution)

