use crate::variables::PRODUCT_NAME;
use serde::Deserialize;
use std::path::PathBuf;

fn resolve_config_path() -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|p| p.join(".config"))
        .map(|p| p.join(PRODUCT_NAME))
        .map(|p| p.join("config.toml"))
        .ok_or_else(|| "failed to resolve path of config file".to_string())
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub connection_string: String,
    pub cache_second: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub redis: Option<RedisConfig>,
}

impl Config {
    pub fn load() -> Result<Config, String> {
        let path = resolve_config_path()?;

        if !path.exists() {
            return Ok(Config { redis: None });
        }

        let text =
            std::fs::read_to_string(path).map_err(|e| format!("failed to read config: {e}"))?;

        let config =
            toml::from_str(&text).map_err(|e| format!("failed to deserialize config: {e}"))?;

        Ok(config)
    }
}
