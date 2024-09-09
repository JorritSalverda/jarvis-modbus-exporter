use jarvis_lib::config_client::SetDefaults;
use jarvis_lib::model::{EntityType, MetricType, SampleType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub location: String,
    pub sample_configs: Vec<ConfigSample>,
}

impl SetDefaults for Config {
    fn set_defaults(&mut self) {}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSample {
    // default jarvis config for sample
    pub entity_type: EntityType,
    pub entity_name: String,
    pub sample_type: SampleType,
    pub sample_name: String,
    pub metric_type: MetricType,

    // modbus specific config for sample
    pub value_multiplier: f64,
    pub register_type: RegisterType,
    pub register_address: u16,
    pub register_quantity: u16,
    #[serde(default)]
    pub signed: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum RegisterType {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "holding")]
    Holding,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RegisterType;
    use jarvis_lib::config_client::{ConfigClient, ConfigClientConfig};
    use jarvis_lib::model::{EntityType, MetricType, SampleType};

    #[test]
    fn read_config_from_file_returns_deserialized_test_file() {
        let config_client =
            ConfigClient::new(ConfigClientConfig::new("test-config.yaml".to_string()).unwrap());

        let config: Config = config_client.read_config_from_file().unwrap();

        assert_eq!(config.location, "My Home".to_string());
        assert_eq!(config.sample_configs.len(), 1);
        assert_eq!(config.sample_configs[0].entity_type, EntityType::Device);
        assert_eq!(
            config.sample_configs[0].entity_name,
            "Sunny TriPower 8.0".to_string()
        );
        assert_eq!(
            config.sample_configs[0].sample_type,
            SampleType::ElectricityProduction
        );
        assert_eq!(
            config.sample_configs[0].sample_name,
            "Totaal opgewekt".to_string()
        );
        assert_eq!(config.sample_configs[0].metric_type, MetricType::Counter);
        assert_eq!(config.sample_configs[0].value_multiplier, 3600f64);
        assert_eq!(config.sample_configs[0].register_type, RegisterType::Input);
        assert_eq!(config.sample_configs[0].register_address, 30513u16);
        assert_eq!(config.sample_configs[0].register_quantity, 4u16);
    }
}
