use std::env;
use std::error::Error;
use std::io;

use crate::config_client::ConfigClient;
use crate::bigquery_client::BigqueryClient;
use crate::state_client::StateClient;
use crate::modbus_client::ModbusClient;

pub struct ExporterServiceConfig {
	config_client:   ConfigClient,
	bigquery_client: BigqueryClient,
	state_client:    StateClient,
	modbus_client:   ModbusClient,
}

impl ExporterServiceConfig {
  pub fn new(config_client:   ConfigClient, bigquery_client: BigqueryClient, state_client:    StateClient, modbus_client:   ModbusClient) -> Result<Self,Box<dyn Error>> {
    Ok(Self{ config_client, bigquery_client, state_client, modbus_client})
  }
}

pub struct ExporterService {
	config:   ExporterServiceConfig,
}

impl ExporterService {
  pub fn new(config:   ExporterServiceConfig) -> Self {
    Self{ config }
  }

  pub fn run(&self, bigquery_init: bool, bigquery_dataset: String, bigquery_table: String) -> Result<bool, io::Error> {

    // // read config from yaml file
    // config, err := s.configClient.ReadConfigFromFile(ctx, *configPath)
    // if err != nil {
    //   return
    // }

    // log.Info().Interface("config", config).Msgf("Loaded config from %v", *configPath)

    // // init bigquery table if it doesn't exist yet
    // if bigqueryInit {
    //   err = s.bigqueryClient.InitBigqueryTable(ctx, bigqueryDataset, bigqueryTable)
    //   if err != nil {
    //     return
    //   }
    // }

    // lastMeasurement, err := s.stateClient.ReadState(ctx)
    // if err != nil {
    //   return
    // }

    // measurement, err := s.modbusClient.GetMeasurement(ctx, config, lastMeasurement)
    // if err != nil {
    //   return
    // }

    // err = s.bigqueryClient.InsertMeasurement(ctx, bigqueryDataset, bigqueryTable, measurement)
    // if err != nil {
    //   return
    // }

    // err = s.stateClient.StoreState(ctx, measurement)
    // if err != nil {
    //   return
    // }

    // return nil

    Ok(true)
  }
}