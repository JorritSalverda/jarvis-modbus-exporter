package main

import (
	"context"

	"github.com/rs/zerolog/log"
)

//go:generate mockgen -package=main -destination ./exporter_service_mock.go -source=exporter_service.go
type ExporterService interface {
	Run(ctx context.Context, bigqueryInit bool, bigqueryDataset, bigqueryTable string) (err error)
}

func NewExporterService(configClient ConfigClient, bigqueryClient BigQueryClient, stateClient StateClient, modbusClient ModbusClient) (ExporterService, error) {
	return &exporterService{
		configClient:   configClient,
		bigqueryClient: bigqueryClient,
		stateClient:    stateClient,
		modbusClient:   modbusClient,
	}, nil
}

type exporterService struct {
	configClient   ConfigClient
	bigqueryClient BigQueryClient
	stateClient    StateClient
	modbusClient   ModbusClient
}

func (s *exporterService) Run(ctx context.Context, bigqueryInit bool, bigqueryDataset, bigqueryTable string) (err error) {

	// read config from yaml file
	config, err := s.configClient.ReadConfigFromFile(ctx, *configPath)
	if err != nil {
		return
	}

	log.Info().Interface("config", config).Msgf("Loaded config from %v", *configPath)

	// init bigquery table if it doesn't exist yet
	if bigqueryInit {
		err = s.bigqueryClient.InitBigqueryTable(ctx, bigqueryDataset, bigqueryTable)
		if err != nil {
			return
		}
	}

	lastMeasurement, err := s.stateClient.ReadState(ctx)
	if err != nil {
		return
	}

	measurement, err := s.modbusClient.GetMeasurement(ctx, config, lastMeasurement)
	if err != nil {
		return
	}

	err = s.bigqueryClient.InsertMeasurement(ctx, bigqueryDataset, bigqueryTable, measurement)
	if err != nil {
		return
	}

	err = s.stateClient.StoreState(ctx, measurement)
	if err != nil {
		return
	}

	return nil
}
