mod modbus_client;
mod model;

use jarvis_lib::config_client::{ConfigClient, ConfigClientConfig};
use jarvis_lib::exporter_service::{ExporterService, ExporterServiceConfig};
use jarvis_lib::nats_client::{NatsClient, NatsClientConfig};
use jarvis_lib::state_client::{StateClient, StateClientConfig};
use modbus_client::{ModbusClient, ModbusClientConfig};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    json_env_logger::init();

    let modbus_client_config = ModbusClientConfig::from_env()?;
    let modbus_client = ModbusClient::new(modbus_client_config);

    let state_client_config = StateClientConfig::from_env().await?;
    let state_client = StateClient::new(state_client_config);

    let nats_client_config = NatsClientConfig::from_env().await?;
    let nats_client = NatsClient::new(nats_client_config);

    let config_client_config = ConfigClientConfig::from_env()?;
    let config_client = ConfigClient::new(config_client_config);

    let exporter_service_config = ExporterServiceConfig::new(
        config_client,
        nats_client,
        state_client,
        Box::new(modbus_client),
    )?;
    let mut exporter_service = ExporterService::new(exporter_service_config);

    exporter_service.run().await?;

    Ok(())
}
