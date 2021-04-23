use std::env;
use std::error::Error;

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

  // func (c *stateClient) ReadState(ctx context.Context) (lastMeasurement *contractsv1.Measurement, err error) {

  //   // check if last measurement file exists in configmap
  //   if _, err := os.Stat(c.measurementFilePath); !os.IsNotExist(err) {
  //     log.Info().Msgf("File %v exists, reading contents...", c.measurementFilePath)
  
  //     // read state file
  //     data, err := ioutil.ReadFile(c.measurementFilePath)
  //     if err != nil {
  //       return lastMeasurement, fmt.Errorf("Failed reading file from path %v: %w", c.measurementFilePath, err)
  //     }
  
  //     log.Info().Msgf("Unmarshalling file %v contents...", c.measurementFilePath)
  
  //     // unmarshal state file
  //     if err := json.Unmarshal(data, &lastMeasurement); err != nil {
  //       return lastMeasurement, fmt.Errorf("Failed unmarshalling last measurement file: %w", err)
  //     }
  //   }
  
  //   return
  // }
  
  // func (c *stateClient) StoreState(ctx context.Context, measurement contractsv1.Measurement) (err error) {
  
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
  // }
  
  // func (c *stateClient) getCurrentNamespace() (namespace string, err error) {
  //   ns, err := ioutil.ReadFile("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
  //   if err != nil {
  //     return namespace, fmt.Errorf("Failed reading namespace: %w", err)
  //   }
  
  //   return string(ns), nil
  // }

}

