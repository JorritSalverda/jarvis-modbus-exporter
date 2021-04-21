use std::time::Duration;
use std::error::Error;
use modbus::Client;
use modbus::tcp;
use uuid::Uuid;
use chrono::{DateTime,Utc};
use byteorder::{ByteOrder,BigEndian};
use conv::*;

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
	id:             String,
	source:         String,
	location:       String,
	samples:        Vec<Sample>,
	measured_at_time: DateTime<Utc>,
}

pub struct Sample {
	entity_type: EntityType,
	entity_name: String,
	sample_type: SampleType,
	sample_name: String,
	metric_type: MetricType,
	value:      f64,
}

pub struct Config {
	location:      String,
	sample_configs: Vec<ConfigSample>,
}

pub struct ConfigSample {
	// default jarvis config for sample
	entity_type: EntityType,
	entity_name: String,
	sample_type: SampleType,
	sample_name: String,
	metric_type: MetricType,

	// modbus specific config for sample
  value_multiplier: f64,
  register_type: RegisterType,
  register_address: u16,
  register_quantity: u16,
}

#[derive(Copy, Clone)]
pub enum RegisterType {
  Input,
  Holding,
}

pub struct ModbusClientConfig {
	host:   String,
	port:   u16,
	unit_id: u8,
}

impl ModbusClientConfig {
  pub fn new<'a>(host: String, port: u16, unit_id: u8) -> Result<ModbusClientConfig, &'a str> {
    if host == "" {
      return Err("Please set the ip address of your modbus device on your local network");
    }
    if port != 502 && (port < 49152 || port > 65535) {
      return Err("Please set the modbus port of your modbus device on your local network to its default 502, or anywhere between 49152 and 65535 if changed in the installer menu");
    }

    Ok(ModbusClientConfig { host, port, unit_id })
  }
}

pub struct ModbusClient {
  config: ModbusClientConfig,
}

impl ModbusClient {
  pub fn new(config: ModbusClientConfig) -> ModbusClient {
    ModbusClient { config }
  }

	pub fn get_measurement(&self, config: Config, last_measurement: Option<Measurement>) -> Result<Measurement, Box<dyn Error>> {

    let mut modbus_client = self.init_modbus_client()?;

    let mut measurement = Measurement{
      id:  Uuid::new_v4().to_string(),
      source:         String::from("jarvis-modbus-exporter"),
      location:       config.location.clone(),
      samples:        Vec::new(),
      measured_at_time: Utc::now(),
    };

    for sample_config in config.sample_configs.iter() {
      match self.get_sample(sample_config, &mut modbus_client) {
        Ok(sample) => { measurement.samples.push(sample); },
        Err(error) => return Err(error),
      };
    }

    Ok(measurement)	
  }

  fn init_modbus_cfg(&self) -> modbus::Config {
    let mut cfg = tcp::Config::default();
    cfg.tcp_port = self.config.port;
    cfg.modbus_uid = self.config.unit_id;
    cfg.tcp_connect_timeout = Some(Duration::new(20, 0));

    cfg
  }

  fn init_modbus_client(&self) -> std::io::Result<modbus::Transport> {
    let cfg = self.init_modbus_cfg();
    
    tcp::Transport::new_with_cfg(&self.config.host, cfg)
  }

	pub fn get_sample(&self, sample_config: &ConfigSample, modbus_client: &mut modbus::Transport) -> Result<Sample, Box<dyn Error>> {
    
    let sample_registers = match sample_config.register_type {
      RegisterType::Input => modbus_client.read_input_registers(sample_config.register_address, sample_config.register_quantity),
      RegisterType::Holding => modbus_client.read_holding_registers(sample_config.register_address, sample_config.register_quantity),
    }?;

    let mut sample_bytes: Vec<u8> = vec![0;sample_registers.len()*2];
    BigEndian::write_u16_into(&sample_registers, &mut sample_bytes);

    let mut sample_value: Vec<u64> = vec![0;sample_registers.len()/4];
    BigEndian::read_u64_into(&sample_bytes, &mut sample_value);
  
 
    // init sample from config
    Ok(Sample{
      entity_type: sample_config.entity_type,
      entity_name: sample_config.entity_name.clone(),
      sample_type: sample_config.sample_type,
      sample_name: sample_config.sample_name.clone(),
      metric_type: sample_config.metric_type,
      value: f64::approx_from(sample_value[0]).unwrap() * sample_config.value_multiplier
    })
  }
}

pub struct StateClient {
	// kubeClientset                *kubernetes.Clientset
	measurement_file_path:          String,
	measurement_file_configmap_name: String,
}

impl StateClient {
  pub fn new(measurement_file_path:          String, measurement_file_configmap_name: String) -> StateClient {
    StateClient { measurement_file_path, measurement_file_configmap_name }
  }
}

pub struct BigQueryClient {
	project_id: String,
	// client    *googlebigquery.Client
	enable:    bool,
}

impl BigQueryClient {
  pub fn new(project_id: String, enable:    bool) -> BigQueryClient {
    BigQueryClient { project_id, enable }
  }
}

pub struct ConfigClient {

}

impl ConfigClient {
  pub fn new() -> ConfigClient {
    ConfigClient{}
  }
}

pub struct ExporterService {
	config_client:   ConfigClient,
	bigquery_client: BigQueryClient,
	state_client:    StateClient,
	modbus_client:   ModbusClient,
}

impl ExporterService {
  pub fn new(config_client:   ConfigClient, bigquery_client: BigQueryClient, state_client:    StateClient, modbus_client:   ModbusClient) -> ExporterService {
    ExporterService{ config_client, bigquery_client, state_client, modbus_client}
  }
}