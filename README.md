# Rust-Static-Analyzer
There is a need for a static analysis tool designed to be run on Rust source code for high assurance and embedded devices. Currently there is only RustC and Clippy that provide any guidance/capabilities for analyzing Rust and neither are robust. 


GOALS
1. Determine rulesets and requirements that need to go into the tool.
2. 2. Find any/all CWE/CVEs against Rust to incorporate into the tool.
3. Need to be platform agnostic so it can be run on both a Linux and a Windows environment.
4. Need to be chip agnostic so it can be run against bare-metal or OS-based implementations.
5. Ideally this would be something developers could implement as part of their CI/CD pipeline to run against code developed as part of regular testing
6. There should be rules that comply with MISRA coding standards as they apply to Rust
7. There will probably be more requirements that are discovered along the way. 
