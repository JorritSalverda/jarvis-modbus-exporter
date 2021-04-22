use std::env;
use std::error::Error;
use std::fs;
use serde_yaml;
use crate::model::{Config,EntityType,SampleType,MetricType,RegisterType};

pub struct ConfigClientConfig {
  config_path: String,
}

impl ConfigClientConfig {
  pub fn new(config_path: String) -> Result<Self, Box<dyn Error>> {
    Ok(Self{ config_path })
  }

  pub fn from_env() -> Result<Self, Box<dyn Error>> {
    let config_path = env::var("CONFIG_PATH").unwrap_or("/configs/config.yaml".to_string());

    Self::new(config_path)
  }
}

pub struct ConfigClient {
  config: ConfigClientConfig
}

impl ConfigClient {
  pub fn new(config: ConfigClientConfig) -> Self {
    Self{ config }
  }

  pub fn read_config_from_file(&self) -> Result<Config, Box<dyn Error>> {
    let config_file_contents = fs::read_to_string(&self.config.config_path)?;
    let config: Config = serde_yaml::from_str(&config_file_contents)?;

    Ok(config)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_config_from_file_returns_deserialized_test_file() {

    let config_client = ConfigClient::new(ConfigClientConfig::new("test-config.yaml".to_string()).unwrap());

    let config = config_client.read_config_from_file().unwrap();

    assert_eq!(config.location, "My Home".to_string());
    assert_eq!(config.sample_configs.len(), 1);
    assert_eq!(config.sample_configs[0].entity_type, EntityType::Device);
    assert_eq!(config.sample_configs[0].entity_name, "Sunny TriPower 8.0".to_string());
    assert_eq!(config.sample_configs[0].sample_type, SampleType::ElectricityProduction);
    assert_eq!(config.sample_configs[0].sample_name, "Totaal opgewekt".to_string());
    assert_eq!(config.sample_configs[0].metric_type, MetricType::Counter);
    assert_eq!(config.sample_configs[0].value_multiplier, 3600f64);
    assert_eq!(config.sample_configs[0].register_type, RegisterType::Input);
    assert_eq!(config.sample_configs[0].register_address, 30513u16);
    assert_eq!(config.sample_configs[0].register_quantity, 4u16);
  }
}