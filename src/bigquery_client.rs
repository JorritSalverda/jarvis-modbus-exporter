use std::env;
use std::error::Error;

use crate::model::{Measurement};

pub struct BigqueryClientConfig {
  google_application_credentials: String,
	project_id: String,
  dataset: String,
  table: String,
	enable:    bool,
  init: bool,

	client:    gcp_bigquery_client::Client,
}

impl BigqueryClientConfig {
  pub async fn new(project_id: String, dataset: String, table: String, google_application_credentials: String, enable:    bool, init: bool) -> Result<Self, Box<dyn Error>> {
    let client = gcp_bigquery_client::Client::from_service_account_key_file(&google_application_credentials).await;

    Ok(Self { google_application_credentials, project_id, dataset, table, enable, init, client })
  }

  pub async fn from_env() -> Result<Self, Box<dyn Error>> {
    let google_application_credentials = env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or(String::from("/secrets/keyfile.json"));
    let project_id = env::var("BQ_PROJECT_ID")?;
    let dataset = env::var("BQ_DATASET")?;
    let table = env::var("BQ_TABLE")?;
    let enable: bool = env::var("BQ_ENABLE").unwrap_or("true".to_string()).parse().unwrap_or(true);
    let init: bool = env::var("BQ_INIT").unwrap_or("true".to_string()).parse().unwrap_or(true);

    Self::new(google_application_credentials, project_id, dataset, table, enable, init ).await
  }  
}

pub struct BigqueryClient {
	config: BigqueryClientConfig,
}

impl BigqueryClient {
  pub fn new(config: BigqueryClientConfig) -> Self {
    Self { config }
  }

	pub fn check_if_dataset_exists(&self, dataset: String) -> Result<bool, Box<dyn Error>> {
    if !self.config.enable{
      return Ok(false)
    }

    // ds := c.client.Dataset(dataset)

    // md, err := ds.Metadata(ctx)

    // log.Error().Err(err).Msgf("Error retrieving metadata for dataset %v", dataset)

    // return md != nil    

    Ok(true)
  }

	pub fn check_if_table_exists(&self, dataset: String, table: String) -> Result<bool, Box<dyn Error>> {
    if !self.config.enable{
      return Ok(false)
    }

    // tbl := c.client.Dataset(dataset).Table(table)

  	// md, _ := tbl.Metadata(ctx)

    Ok(true)
  }

	pub fn create_table(&self, dataset: String, table: String, partition_field: String, wait_ready: bool) -> Result<bool, Box<dyn Error>> {
    if !self.config.enable{
      return Ok(false)
    }

    // tbl := c.client.Dataset(dataset).Table(table)

    // // infer the schema of the type
    // schema, err := googlebigquery.InferSchema(typeForSchema)
    // if err != nil {
    //   return err
    // }

    // tableMetadata := &googlebigquery.TableMetadata{
    //   Schema: schema,
    // }

    // // if partitionField is set use it for time partitioning
    // if partitionField != "" {
    //   tableMetadata.TimePartitioning = &googlebigquery.TimePartitioning{
    //     Field: partitionField,
    //   }
    // }

    // // create the table
    // err = tbl.Create(ctx, tableMetadata)
    // if err != nil {
    //   return err
    // }

    // if waitReady {
    //   for {
    //     if c.CheckIfTableExists(ctx, dataset, table) {
    //       break
    //     }
    //     time.Sleep(time.Second)
    //   }
    // }

    Ok(true)
  }

	pub fn update_table_schema(&self, dataset: String, table: String) -> Result<bool, Box<dyn Error>> {
    if !self.config.enable{
      return Ok(false)
    }

    // tbl := c.client.Dataset(dataset).Table(table)

    // // infer the schema of the type
    // schema, err := googlebigquery.InferSchema(typeForSchema)
    // if err != nil {
    //   return err
    // }

    // meta, err := tbl.Metadata(ctx)
    // if err != nil {
    //   return err
    // }

    // update := googlebigquery.TableMetadataToUpdate{
    //   Schema: schema,
    // }
    // if _, err := tbl.Update(ctx, update, meta.ETag); err != nil {
    //   return err
    // }

    Ok(true)
  }

	pub fn insert_measurement(&self, dataset: String, table: String, measurement: Measurement) -> Result<bool, Box<dyn Error>> {
    if !self.config.enable{
      return Ok(false)
    }

    // tbl := c.client.Dataset(dataset).Table(table)

    // u := tbl.Uploader()

    // if err := u.Put(context.Background(), measurement); err != nil {
    //   return err
    // }

    Ok(true)
  }

	pub fn init_table(&self, dataset: String, table: String) -> Result<bool, Box<dyn Error>> {
    if !self.config.enable{
      return Ok(false)
    }

    // log.Debug().Msgf("Checking if table %v.%v.%v exists...", c.projectID, dataset, table)
    // tableExist := c.CheckIfTableExists(ctx, dataset, table)

    // if !tableExist {
    //   log.Debug().Msgf("Creating table %v.%v.%v...", c.projectID, dataset, table)
    //   err := c.CreateTable(ctx, dataset, table, contractsv1.Measurement{}, "MeasuredAtTime", true)
    //   if err != nil {
    //     return fmt.Errorf("Failed creating bigquery table: %w", err)
    //   }
    // } else {
    //   log.Debug().Msgf("Trying to update table %v.%v.%v schema...", c.projectID, dataset, table)
    //   err := c.UpdateTableSchema(ctx, dataset, table, contractsv1.Measurement{})
    //   if err != nil {
    //     return fmt.Errorf("Failed updating bigquery table schema: %w", err)
    //   }
    // }

    Ok(true)
  }
}
