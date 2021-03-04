package main

import (
	"context"
	"fmt"
	"time"

	googlebigquery "cloud.google.com/go/bigquery"
	contractsv1 "github.com/JorritSalverda/jarvis-contracts-golang/contracts/v1"
	"github.com/rs/zerolog/log"
)

// BigQueryClient is the interface for connecting to bigquery
//go:generate mockgen -package=main -destination ./bigquery_client_mock.go -source=bigquery_client.go
type BigQueryClient interface {
	CheckIfDatasetExists(ctx context.Context, dataset string) (exists bool)
	CheckIfTableExists(ctx context.Context, dataset, table string) (exists bool)
	CreateTable(ctx context.Context, dataset, table string, typeForSchema interface{}, partitionField string, waitReady bool) (err error)
	UpdateTableSchema(ctx context.Context, dataset, table string, typeForSchema interface{}) (err error)
	DeleteTable(ctx context.Context, dataset, table string) (err error)
	InsertMeasurement(ctx context.Context, dataset, table string, measurement contractsv1.Measurement) (err error)
	InitBigqueryTable(ctx context.Context, dataset, table string) (err error)
}

// NewBigQueryClient returns new BigQueryClient
func NewBigQueryClient(ctx context.Context, projectID string, enable bool) (BigQueryClient, error) {

	internalBigqueryClient, err := googlebigquery.NewClient(ctx, projectID)
	if err != nil {
		return nil, err
	}

	return &bigqueryClient{
		projectID: projectID,
		client:    internalBigqueryClient,
		enable:    enable,
	}, nil
}

type bigqueryClient struct {
	projectID string
	client    *googlebigquery.Client
	enable    bool
}

func (c *bigqueryClient) CheckIfDatasetExists(ctx context.Context, dataset string) (exists bool) {

	if !c.enable {
		return false
	}

	ds := c.client.Dataset(dataset)

	md, err := ds.Metadata(ctx)

	log.Error().Err(err).Msgf("Error retrieving metadata for dataset %v", dataset)

	return md != nil
}

func (c *bigqueryClient) CheckIfTableExists(ctx context.Context, dataset, table string) (exists bool) {

	if !c.enable {
		return false
	}

	tbl := c.client.Dataset(dataset).Table(table)

	md, _ := tbl.Metadata(ctx)

	// log.Error().Err(err).Msgf("Error retrieving metadata for table %v", table)

	return md != nil
}

func (c *bigqueryClient) CreateTable(ctx context.Context, dataset, table string, typeForSchema interface{}, partitionField string, waitReady bool) (err error) {

	if !c.enable {
		return nil
	}

	tbl := c.client.Dataset(dataset).Table(table)

	// infer the schema of the type
	schema, err := googlebigquery.InferSchema(typeForSchema)
	if err != nil {
		return err
	}

	tableMetadata := &googlebigquery.TableMetadata{
		Schema: schema,
	}

	// if partitionField is set use it for time partitioning
	if partitionField != "" {
		tableMetadata.TimePartitioning = &googlebigquery.TimePartitioning{
			Field: partitionField,
		}
	}

	// create the table
	err = tbl.Create(ctx, tableMetadata)
	if err != nil {
		return err
	}

	if waitReady {
		for {
			if c.CheckIfTableExists(ctx, dataset, table) {
				break
			}
			time.Sleep(time.Second)
		}
	}

	return nil
}

func (c *bigqueryClient) UpdateTableSchema(ctx context.Context, dataset, table string, typeForSchema interface{}) (err error) {

	if !c.enable {
		return nil
	}

	tbl := c.client.Dataset(dataset).Table(table)

	// infer the schema of the type
	schema, err := googlebigquery.InferSchema(typeForSchema)
	if err != nil {
		return err
	}

	meta, err := tbl.Metadata(ctx)
	if err != nil {
		return err
	}

	update := googlebigquery.TableMetadataToUpdate{
		Schema: schema,
	}
	if _, err := tbl.Update(ctx, update, meta.ETag); err != nil {
		return err
	}

	return nil
}

func (c *bigqueryClient) DeleteTable(ctx context.Context, dataset, table string) (err error) {

	if !c.enable {
		return nil
	}

	tbl := c.client.Dataset(dataset).Table(table)

	// delete the table
	err = tbl.Delete(ctx)
	if err != nil {
		return err
	}

	return nil
}

func (c *bigqueryClient) InsertMeasurement(ctx context.Context, dataset, table string, measurement contractsv1.Measurement) (err error) {

	if !c.enable {
		return nil
	}

	tbl := c.client.Dataset(dataset).Table(table)

	u := tbl.Uploader()

	if err := u.Put(context.Background(), measurement); err != nil {
		return err
	}

	return nil
}

func (c *bigqueryClient) InitBigqueryTable(ctx context.Context, dataset, table string) (err error) {

	log.Debug().Msgf("Checking if table %v.%v.%v exists...", c.projectID, dataset, table)
	tableExist := c.CheckIfTableExists(ctx, dataset, table)

	if !tableExist {
		log.Debug().Msgf("Creating table %v.%v.%v...", c.projectID, dataset, table)
		err := c.CreateTable(ctx, dataset, table, contractsv1.Measurement{}, "MeasuredAtTime", true)
		if err != nil {
			return fmt.Errorf("Failed creating bigquery table: %w", err)
		}
	} else {
		log.Debug().Msgf("Trying to update table %v.%v.%v schema...", c.projectID, dataset, table)
		err := c.UpdateTableSchema(ctx, dataset, table, contractsv1.Measurement{})
		if err != nil {
			return fmt.Errorf("Failed updating bigquery table schema: %w", err)
		}
	}

	return nil
}
