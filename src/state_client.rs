use std::env;
use std::error::Error;

pub struct StateClientConfig {
	// kubeClientset                *kubernetes.Clientset
	measurement_file_path:          String,
	measurement_file_configmap_name: String,
}

impl StateClientConfig {
  pub fn new(measurement_file_path: String, measurement_file_configmap_name: String) -> Result<StateClientConfig,Box<dyn Error>> {
    Ok(StateClientConfig { measurement_file_path, measurement_file_configmap_name })
  }

  pub fn from_env() -> Result<StateClientConfig, Box<dyn Error>> {
    let measurement_file_path = env::var("MEASUREMENT_FILE_PATH").unwrap_or("/configs/last-measurement.json".to_string());
    let measurement_file_configmap_name = env::var("MEASUREMENT_FILE_CONFIG_MAP_NAME").unwrap_or("jarvis-modbus-exporter".to_string());

    StateClientConfig::new(measurement_file_path, measurement_file_configmap_name)
  }
}

pub struct StateClient {
	// kubeClientset                *kubernetes.Clientset
	config: StateClientConfig,
}

impl StateClient {
  pub fn new(config: StateClientConfig) -> StateClient {
    StateClient { config }
  }
}

