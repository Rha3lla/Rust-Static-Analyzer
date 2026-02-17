use crate::cli::Args;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    pub profile: Option<String>,
    pub format: Option<String>,
    pub deny: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EffectiveConfig {
    pub profile: String,
    pub format: String,
    pub deny: String,
}

pub fn load_config(args: &Args) -> Result<EffectiveConfig> {
    // Defaults
    let mut cfg = EffectiveConfig {
        profile: "high_assurance".to_string(),
        format: "text".to_string(),
        deny: "error".to_string(),
    };

    // Determine config path
    let config_path = if let Some(p) = &args.config {
        Some(p.clone())
    } else {
        // Auto-discover: ha-lint.toml in current dir if present
        let p = "ha-lint.toml";
        if std::path::Path::new(p).exists() { Some(p.to_string()) } else { None }
    };

    if let Some(path) = config_path {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {path}"))?;
        let parsed: ConfigFile = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML config: {path}"))?;

        if let Some(p) = parsed.profile { cfg.profile = p; }
        if let Some(f) = parsed.format { cfg.format = f; }
        if let Some(d) = parsed.deny { cfg.deny = d; }
    }

    Ok(cfg)
}
