use std::env;
use std::error::Error;
use std::fs;
use crate::model::{Measurement};

pub struct StateClientConfig {
	// kubeClientset                *kubernetes.Clientset
	measurement_file_path:          String,
	measurement_file_configmap_name: String,
}

impl StateClientConfig {
  pub fn new(measurement_file_path: String, measurement_file_configmap_name: String) -> Result<Self,Box<dyn Error>> {
    Ok(Self { measurement_file_path, measurement_file_configmap_name })
  }

  pub fn from_env() -> Result<Self, Box<dyn Error>> {
    let measurement_file_path = env::var("MEASUREMENT_FILE_PATH").unwrap_or("/configs/last-measurement.json".to_string());
    let measurement_file_configmap_name = env::var("MEASUREMENT_FILE_CONFIG_MAP_NAME").unwrap_or("jarvis-modbus-exporter".to_string());

    Self::new(measurement_file_path, measurement_file_configmap_name)
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
  
  pub fn store_state(&self, measurement: &Measurement) ->  Result<(), Box<dyn std::error::Error>> {
  
  //   currentNamespace, err := c.getCurrentNamespace()
  //   if err != nil {
  //     return
  //   }
  
  //   // retrieve configmap
  //   configMap, err := c.kubeClientset.CoreV1().ConfigMaps(currentNamespace).Get(ctx, c.measurementFileConfigMapName, metav1.GetOptions{})
  //   if err != nil {
  //     return fmt.Errorf("Failed retrieving configmap %v: %w", c.measurementFileConfigMapName, err)
  //   }
  
  //   // marshal state to json
  //   measurementData, err := json.Marshal(measurement)
  //   if configMap.Data == nil {
  //     configMap.Data = make(map[string]string)
  //   }
  
  //   configMap.Data[filepath.Base(c.measurementFilePath)] = string(measurementData)
  
  //   // update configmap to have measurement available when the application runs the next time and for other applications
  //   _, err = c.kubeClientset.CoreV1().ConfigMaps(currentNamespace).Update(ctx, configMap, metav1.UpdateOptions{})
  //   if err != nil {
  //     return fmt.Errorf("Failed updating configmap %v: %w", c.measurementFileConfigMapName, err)
  //   }
  
  //   log.Info().Msgf("Stored measurement in configmap %v...", c.measurementFileConfigMapName)
  
  //   return nil
    Ok(())
  }
  
  // func (c *stateClient) getCurrentNamespace() (namespace string, err error) {
  //   ns, err := ioutil.ReadFile("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
  //   if err != nil {
  //     return namespace, fmt.Errorf("Failed reading namespace: %w", err)
  //   }
  
  //   return string(ns), nil
  // }

}

