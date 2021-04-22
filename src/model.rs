use chrono::{DateTime,Utc};

#[derive(Copy, Clone)]
pub enum EntityType {
  Invalid,
  Tariff,
  Zone,
  Device,
}

#[derive(Copy, Clone)]
pub enum MetricType {
  Invalid,
  Counter,
  Gauge,
}

#[derive(Copy, Clone)]
pub enum SampleType {
  Invalid,
  ElectricityConsumption,
  ElectricityProduction,
  GasConsumption,
  Temperature,
  Pressure,
  Flow,
  Humidity,
  Time,
}

pub struct Measurement {
	pub id:             String,
	pub source:         String,
	pub location:       String,
	pub samples:        Vec<Sample>,
	pub measured_at_time: DateTime<Utc>,
}

pub struct Sample {
	pub entity_type: EntityType,
	pub entity_name: String,
	pub sample_type: SampleType,
	pub sample_name: String,
	pub metric_type: MetricType,
	pub value:      f64,
}

pub struct Config {
	pub location:      String,
	pub sample_configs: Vec<ConfigSample>,
}

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

#[derive(Copy, Clone)]
pub enum RegisterType {
  Input,
  Holding,
}