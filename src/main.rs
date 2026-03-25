mod find;
use find::find_config;

mod be1;
use be1::tokenize;
use tokenize::{Token, TokenStream};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

use std::{fs::read_to_string, path::PathBuf};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct FmtConfig {
    tab_size: u8,
    column_limit: u8,
    declaration_spacing: u8,
}

#[derive(Parser, Debug)]
#[command(version, about = "Format source files", long_about = None)]
struct Args {
    #[arg(value_name = "FILE")]
    formatted_file: PathBuf,

    #[arg(short, long)]
    inplace: bool,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        if self.inplace && self.out.is_some() {
            return Err(anyhow!("Cannot use --inplace with --out"));
        }
        Ok(())
    }
}

const DEFAULT_CONFIG: FmtConfig = FmtConfig {
    tab_size: 4,
    column_limit: 120,
    declaration_spacing: 1,
};

fn validate_config(config: &FmtConfig) -> Result<()> {
    let mut config_errors = Vec::new();
    if !matches!(config.tab_size, 2 | 4 | 8) {
        config_errors.push(format!(
            "You used a column limit of {}; the only acceptable tab sizes are 2, 4, and 8",
            config.tab_size
        ));
    }
    if config.column_limit > 120 {
        config_errors.push(format!(
            "You used a column limit of {}; column limit cannot exceed 120 characters",
            config.column_limit
        ));
    }

    if !config_errors.is_empty() {
        return Err(anyhow!("Config errors:\n{}", config_errors.join("\n")));
    }
    Ok(())
}

fn load_config(config_path: Option<PathBuf>) -> Result<FmtConfig> {
    config_path
        .or_else(find_config)
        .map(|path| {
            let config_str = read_to_string(&path)
                .with_context(|| format!("Failed to read config at {}", path.display()))?;
            toml::from_str(&config_str)
                .with_context(|| format!("Failed to parse config at {}", path.display()))
        })
        .unwrap_or(Ok(DEFAULT_CONFIG))
}

fn main() -> Result<()> {
    let args = Args::parse();
    args.validate()?;

    let config = load_config(args.config)?;

    validate_config(&config)?;

    let source =
        read_to_string(&args.formatted_file).with_context(|| "Failed to read file to format")?;
    let tokens = TokenStream::from_source(&source);
    Ok(())
}
