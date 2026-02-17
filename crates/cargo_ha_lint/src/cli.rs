use clap::{ArgAction, Parser};

/// cargo ha-lint (prototype)
#[derive(Debug, Parser)]
#[command(name = "cargo ha-lint")]
#[command(author, version, about = "Static analysis for high-assurance and embedded Rust.")]
pub struct Args {
    /// Select profile (embedded|high_assurance|security|misra_rust)
    #[arg(long)]
    pub profile: Option<String>,

    /// Output format (text|json|sarif)
    #[arg(long)]
    pub format: Option<String>,

    /// Write output to a file (default: stdout)
    #[arg(long)]
    pub output: Option<String>,

    /// Path to config file (default: auto-discover ha-lint.toml in workspace root)
    #[arg(long)]
    pub config: Option<String>,

    /// Fail if findings at/above level exist (note|warning|error)
    #[arg(long)]
    pub deny: Option<String>,

    /// Only emit machine output (no extra logs)
    #[arg(long, action=ArgAction::SetTrue)]
    pub quiet: bool,

    /// Increase logging (-v, -vv)
    #[arg(short, long, action=ArgAction::Count)]
    pub verbose: u8,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
