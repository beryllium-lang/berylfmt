mod find;
use find::find_config;

use serde::{Serialize, Deserialize};
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct FmtConfig {
    tab_size: u8,
    column_limit: u8,
    declaration_spacing: u8
}

const DEFAULT_CONFIG: FmtConfig = FmtConfig {
    tab_size: 4,
    column_limit: 120,
    declaration_spacing: 1
};

fn validate_config(config: &FmtConfig) -> Result<(), String> {
    let mut config_errors = Vec::new();
    match config.tab_size {
        2 | 4 | 8 => {},
        _ => config_errors.push(format!("You used a tab size of {}; the only acceptable tab sizes are 2, 4, and 8 spaces", config.tab_size))
    }

    if config.column_limit > 120 {
        config_errors.push(format!("You used a column limit of {}; column limit cannot exceed 120 characters", config.column_limit));
    }

    if !config_errors.is_empty() {
        return Err(format!("Config errors:\n{}", config_errors.join("\n")));
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let config = match find_config() {
        Some(path) => {
            let config_str = read_to_string(&path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            toml::from_str(&config_str)
                .map_err(|e| format!("Failed to parse config: {}", e))?
        },
        None => DEFAULT_CONFIG
    };

    validate_config(&config)?;
    Ok(())
}
