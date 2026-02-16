use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Note,
    Warning,
    Error,
}

impl Severity {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "note" => Some(Self::Note),
            "warning" | "warn" => Some(Self::Warning),
            "error" | "err" => Some(Self::Error),
            _ => None,
            }
        }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Note => "note",
            Severity::Warning => "warning",
            Severity::Error => "error",
            }
        }
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub file: String,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub rule_id: String,
    pub severity: Severity,
    pub message: String,
    pub span: Option<Span>,
    pub help_uri: Option<String>,
    pub cwe: Vec<String>,
    pub profiles: Vec<String>,
}

////output from a single analysis run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub tool_version: String,
    pub profile: String,
    pub diagnostics: Vec<Diagnostic>,
}

impl AnalysisResult {
    pub fn max_severity(&self) -> Option<Severity> {
        self.diagnostics.iter().map(|d| d.severity).max()
        }
    }