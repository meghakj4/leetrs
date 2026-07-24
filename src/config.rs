use anyhow::{Context, Result};
use std::{fs, sync::OnceLock};

use serde::{Deserialize, Serialize};

use crate::get_config_file;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub editor: Option<String>,
    pub language: Option<String>,
    pub show_description: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: Some("nvim".to_string()),
            language: Some("python3".to_string()),
            show_description: Some(true),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_path = get_config_file();
        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create config directory at {:?}", parent)
                })?;
            }

            let default_config = Self::default();
            let default_toml = toml::to_string_pretty(&default_config)
                .context("Failed to serialize default config to TOML")?;

            fs::write(&config_path, default_toml).with_context(|| {
                format!("Failed to create default config file at {:?}", config_path)
            })?;
        }
        let config_data = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let parsed: Self = toml::from_str(&config_data).context("Failed to parse TOML config")?;
        Ok(parsed)
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
