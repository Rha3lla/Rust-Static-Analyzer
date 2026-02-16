mod cli;
mod config;
mod output;

use anyhow::{Context, Result};
use cli::Args;
use ha_lint_core::engine::{run_analysis, EngineOptions};
use ha_lint_core::model::Severity;

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ha_lint: {}", e);
        std::process::exit(2);
        }
    }

fn real_main() -> Result<()> {
    let args = Args::parse();

    let cfg = config::load_config(&args)?;

    //merge cli overrides config, config overrides default
    let profile = args.profile.clone().unwrap_or_else(|| cfg.profile.clone());
    let format = args.format.clone().unwrap_or_else(|| cfg.format.clone());
    let deny = args.deny.clone().unwrap_or_else(|| cfg.deny.clone());

    let deny_level = Severity::parse(&deny)
    .with_context(|| format!("Invalid --deny level: {deny}. Use note|warning|error."))?;

    let opts = EngineOptions {
        profile: profile.clone(),
        tool_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let mut result = run_analysis(&opts);

        //Deterministic ordering: file, line, col, rule_id
        result.diagnostics.sort_by(|a, b| {
            let aspan = a.span.as_ref();
            let bspan = b.span.as_ref();
            match (aspan, bspan) {
                (Some(sa), Some(sb)) => (sa.file.as_str(), sa.line, sa.col, a.rule_id.as_str())
                .cmp(&(sb.file.as_str(), sb.line, sb.col, b.rule_id.as_str()))
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.rule_id,cmp(&b.rule_id),
                }
            });

            output::emit(&result, &format, args.output.as_deref(), args.quiet)?;

            //exit codes:
            //0 = no findings at/above deny
            //1 = findings at/above deny
            //2 = tool error (handled in main)
            let should_fail = result
            .diagnostics
            .iter()
            .any(|d| d.severity >= deny_level);

            if should_fail {
                std::process::exit(1);
                }
            Ok(())

        }
