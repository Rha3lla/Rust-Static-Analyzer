use crate::model::{AnalysisResult, Diagnostic, Severity, Span};

#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub profile: String,
    pub tool_version: String,
}

pub fn run_analysis(_opts: &EngineOptions) -> AnalysisResult {
    // Stub: return empty results for now.
    // Later this will invoke rustc_driver / HIR/MIR analysis.
    //
    // If you want to confirm pipeline behavior, uncomment the sample diagnostic below.

    // let sample = Diagnostic {
    //     rule_id: "HA-RUST-0001".to_string(),
    //     severity: Severity::Error,
    //     message: "Unjustified unsafe block (sample)".to_string(),
    //     span: Some(Span { file: "src/lib.rs".to_string(), line: 1, col: 1 }),
    //     help_uri: Some("docs/rules/HA-RUST-0001.md".to_string()),
    //     cwe: vec!["CWE-119".to_string()],
    //     profiles: vec!["high_assurance".to_string()],
    // };

    AnalysisResult {
        tool_version: _opts.tool_version.clone(),
        profile: _opts.profile.clone(),
        diagnostics: vec![
            // sample
        ],
    }
}
