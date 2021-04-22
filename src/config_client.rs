use std::env;
use std::error::Error;

pub struct ConfigClientConfig {
  config_path: String,
}

impl ConfigClientConfig {
  pub fn new(config_path: String) -> Result<ConfigClientConfig, Box<dyn Error>> {
    Ok(ConfigClientConfig{ config_path })
  }

  pub fn from_env() -> Result<ConfigClientConfig, Box<dyn Error>> {
    let config_path = env::var("CONFIG_PATH").unwrap_or("/configs/config.yaml".to_string());

    ConfigClientConfig::new(config_path)
  }
}

pub struct ConfigClient {
  config: ConfigClientConfig
}

impl ConfigClient {
  pub fn new(config: ConfigClientConfig) -> ConfigClient {
    ConfigClient{ config }
  }
}
