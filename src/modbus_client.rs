use std::time::Duration;
use std::env;
use std::error::Error;
use modbus::Client;
use modbus::tcp;
use uuid::Uuid;
use chrono::Utc;
use byteorder::{ByteOrder,BigEndian};
use conv::*;
use crate::model::{Config,ConfigSample,Measurement,Sample,RegisterType};

pub struct ModbusClientConfig {
	host:   String,
	port:   u16,
	unit_id: u8,
}

impl ModbusClientConfig {
  pub fn new(host: String, port: u16, unit_id: u8) -> Result<ModbusClientConfig, Box<dyn Error>> {
    if host == "" {
      return Err(Box::<dyn Error>::from("Please set the ip address of your modbus device on your local network"));
    }
    if port != 502 && (port < 49152) {
      return Err(Box::<dyn Error>::from("Please set the modbus port of your modbus device on your local network to its default 502, or anywhere between 49152 and 65535 if changed in the installer menu"));
    }

    Ok(ModbusClientConfig { host, port, unit_id })
  }

  pub fn from_env() -> Result<ModbusClientConfig, Box<dyn Error>> {
    let host = env::var("MODBUS_HOST_IP").unwrap_or("127.0.0.1".to_string());
    let port: u16 = env::var("MODBUS_HOST_PORT").unwrap_or("502".to_string()).parse().unwrap_or(502);
    let unit_id: u8 = env::var("MODBUS_UNIT_ID").unwrap_or("3".to_string()).parse().unwrap_or(3);

    ModbusClientConfig::new(host, port, unit_id)
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