use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::fs;
use toml;

const CONFIG_FILE_PATH: &str = "config.toml";

#[derive(Deserialize, Debug)]
pub struct Config {}

pub fn generate_config() -> Result<Config> {
    let Ok(raw_config) = fs::read_to_string(CONFIG_FILE_PATH) else {
        return Err(anyhow!("Could not read config.toml at {CONFIG_FILE_PATH}"));
    };

    let config: Config = toml::from_str(&raw_config)?;

    return Ok(config);
}
