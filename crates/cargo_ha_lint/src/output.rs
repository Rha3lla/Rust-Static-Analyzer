use anyhow::{Context, Result};
use ha_lint_core::model::AnalysisResult;
use std::io::Write;

pub fn emit(result: &AnalysisResult, format: &str, output_path: Option<&str>, quiet: bool) -> Result<()> {
    let mut out: Box<dyn Write> = match output_path {
        Some(path) => Box::new(std::fs::File::create(path)
            .with_context(|| format!("Failed to create output file: {path}"))?),
        None => Box::new(std::io::stdout()),
    };

    match format.to_ascii_lowercase().as_str() {
        "text" => emit_text(result, &mut out, quiet),
        "json" => emit_json(result, &mut out),
        "sarif" => emit_sarif_stub(result, &mut out),
        other => anyhow::bail!("Unknown --format: {other}. Use text|json|sarif."),
    }
}

fn emit_text(result: &AnalysisResult, out: &mut dyn Write, quiet: bool) -> Result<()> {
    // quiet only affects extra logs (we currently don’t print extra logs),
    // but keep the hook for later.
    let _ = quiet;

    for d in &result.diagnostics {
        if let Some(span) = &d.span {
            writeln!(
                out,
                "{} {} {}  {}:{}:{}",
                d.severity.as_str().to_ascii_uppercase(),
                d.rule_id,
                d.message,
                span.file,
                span.line,
                span.col
            )?;
        } else {
            writeln!(
                out,
                "{} {} {}",
                d.severity.as_str().to_ascii_uppercase(),
                d.rule_id,
                d.message
            )?;
        }
    }
    Ok(())
}

fn emit_json(result: &AnalysisResult, out: &mut dyn Write) -> Result<()> {
    serde_json::to_writer_pretty(out, result)?;
    writeln!(out)?;
    Ok(())
}

/// v0.1: SARIF placeholder so the CLI contract exists.
/// We’ll replace this with a real SARIF emitter in the reporting milestone.
fn emit_sarif_stub(result: &AnalysisResult, out: &mut dyn Write) -> Result<()> {
    // Minimal placeholder: valid JSON but not SARIF-spec compliant yet.
    // This avoids breaking the CLI later.
    let stub = serde_json::json!({
        "note": "SARIF output is not implemented yet",
        "tool": "cargo-ha-lint",
        "version": result.tool_version,
        "profile": result.profile,
        "diagnostics": result.diagnostics,
    });
    serde_json::to_writer_pretty(out, &stub)?;
    writeln!(out)?;
    Ok(())
}
