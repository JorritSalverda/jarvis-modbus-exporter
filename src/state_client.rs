use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::BTreeMap;
use crate::model::{Measurement};
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{api::{Api, PostParams}, Client};

pub struct StateClientConfig {
  kube_client: kube::Client,
	measurement_file_path:          String,
	measurement_file_configmap_name: String,
}

impl StateClientConfig {
  pub fn new(kube_client: kube::Client, measurement_file_path: String, measurement_file_configmap_name: String) -> Result<Self,Box<dyn Error>> {
    Ok(Self { kube_client, measurement_file_path, measurement_file_configmap_name })
  }

  pub async fn from_env() -> Result<Self, Box<dyn Error>> {
    let kube_client: kube::Client = Client::try_default().await?;

    let measurement_file_path = env::var("MEASUREMENT_FILE_PATH").unwrap_or("/configs/last-measurement.json".to_string());
    let measurement_file_configmap_name = env::var("MEASUREMENT_FILE_CONFIG_MAP_NAME").unwrap_or("jarvis-modbus-exporter".to_string());

    Self::new(kube_client, measurement_file_path, measurement_file_configmap_name)
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

  pub fn read_state(&self) -> Result<Option<Measurement>, Box<dyn std::error::Error>> {

    let state_file_contents = fs::read_to_string(&self.config.measurement_file_path)?;
    let last_measurement: Option<Measurement> = match serde_json::from_str(&state_file_contents){
      Ok(lm) => Some(lm),
      Err(_) => None,
    };

    Ok(last_measurement)
  }
  
  pub async fn store_state(&self, measurement: &Measurement) ->  Result<(), Box<dyn std::error::Error>> {
  
    let namespace = &self.get_current_namespace()?;
    
    // retrieve configmap
    let configmaps_api : Api<ConfigMap> = Api::namespaced(self.config.kube_client.clone(), &namespace);
    let mut config_map = configmaps_api.get(&self.config.measurement_file_configmap_name).await?;

    // marshal state to json
    let json_data = match serde_json::to_string(measurement){
      Ok(js) => js,
      Err(e) => return Err(Box::new(e)),
    };
    
    // extract filename from config file path
    let measurement_file_path = Path::new(&self.config.measurement_file_path);
    let measurement_file_name = match measurement_file_path.file_name() {
      Some(filename) => match filename.to_str(){
        Some(filename) => String::from(filename),
        None => return Err(Box::<dyn Error>::from("No filename found in path")),
      },
      None => return Err(Box::<dyn Error>::from("No filename found in path")),
    };

    // update data in configmap
    let mut data = match config_map.data {
      Some(d) => d,
      None => BTreeMap::new(),
    };
    data.insert(measurement_file_name, json_data);
    config_map.data = Some(data);

    // update configmap to have measurement available when the application runs the next time and for other applications
    configmaps_api.replace(&self.config.measurement_file_configmap_name, &PostParams::default(), &config_map).await?;

    Ok(())
  }
  
  fn get_current_namespace(&self) -> Result<String, Box<dyn std::error::Error>> {
    let namespace = fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace")?;

    Ok(namespace)
  }
}
