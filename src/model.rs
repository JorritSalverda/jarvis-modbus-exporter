use jarvis_lib::{EntityType, MetricType, SampleType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub location: String,
    pub sample_configs: Vec<ConfigSample>,
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
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum RegisterType {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "holding")]
    Holding,
}
