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

fn main() -> Result<(), String> {
    let path = find_config();

    let mut config: FmtConfig;
    match path {
        Some(path) => {
            let config_str = read_to_string(&path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            config = toml::from_str(&config_str)
                .map_err(|e| format!("Failed to parse config: {}", e))?;
    
            let mut config_errors = Vec::new();
            config_errors.reserve(2);
            match config.tab_size {
                2 | 4 | 8 => {},
                _ => config_errors.push(format!("You used a tab size of {}; the only acceptable tab sizes are 2, 4, and 8 spaces", config.tab_size))
            }

            if config.column_limit > 120 {
                config_errors.push(format!("You used a column limit of {}; column limit cannot exceed 120 characters", config.column_limit));
            }

            if !config_errors.is_empty() {
                let mut err_str = String::from("Config errors:\n");
                for err in config_errors {
                    err_str += &err;
                    err_str.push('\n');
                }
                return Err(err_str);
            }
        },
        None => {
            config = FmtConfig {
                tab_size: 4,
                column_limit: 120,
                declaration_spacing: 1
            };
       }
    }
    Ok(())
}
