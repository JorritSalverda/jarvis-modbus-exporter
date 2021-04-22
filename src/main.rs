mod bigquery_client;
mod config_client;
mod model;
mod exporter_service;
mod modbus_client;
mod state_client;

use std::process;
use std::env;
use futures::executor::block_on;

use bigquery_client::{BigqueryClientConfig,BigqueryClient};
use config_client::{ConfigClientConfig,ConfigClient};
use exporter_service::{ExporterServiceConfig,ExporterService};
use modbus_client::{ModbusClientConfig,ModbusClient};
use state_client::{StateClientConfig,StateClient};

fn main() {

    let modbus_client_config = ModbusClientConfig::from_env().unwrap_or_else(|err| {
      println!("Failed parsing ModbusClientConfig: {}", err);
      process::exit(1);
    });

    let state_client_config = StateClientConfig::from_env().unwrap_or_else(|err| {
      println!("Failed parsing StateClientConfig: {}", err);
      process::exit(1);
    });

    let bigquery_client_config = block_on(BigqueryClientConfig::from_env()).unwrap_or_else(|err| {
      println!("Failed parsing BigqueryClientConfig: {}", err);
      process::exit(1);
    });

    let config_client_config = ConfigClientConfig::from_env().unwrap_or_else(|err| {
      println!("Failed parsing ConfigClientConfig: {}", err);
      process::exit(1);
    });

    let modbus_client = ModbusClient::new(modbus_client_config);
    let state_client = StateClient::new(state_client_config);
    let bigquery_client = BigqueryClient::new(bigquery_client_config);
    let config_client = ConfigClient::new(config_client_config);

    let exporter_service_config = ExporterServiceConfig::new(config_client, bigquery_client, state_client, modbus_client).unwrap_or_else(|err| {
      println!("Failed parsing ExporterServiceConfig: {}", err);
      process::exit(1);
    });

    let exporter_service = ExporterService::new(exporter_service_config);

    // exporter_service_config.run()
}
