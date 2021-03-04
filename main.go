package main

import (
	"context"
	"runtime"

	"github.com/alecthomas/kingpin"
	foundation "github.com/estafette/estafette-foundation"
	"github.com/rs/zerolog/log"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/rest"
)

var (
	// set when building the application
	appgroup  string
	app       string
	version   string
	branch    string
	revision  string
	buildDate string
	goVersion = runtime.Version()

	// application specific config
	modbusHostIPAddress = kingpin.Flag("modbus-host-ip", "Host ip address ofmodbus").Default("127.0.0.1").OverrideDefaultFromEnvar("MODBUS_HOST_IP").String()
	modbusHostPort      = kingpin.Flag("modbus-host-port", "Host port of modbus").Default("502").OverrideDefaultFromEnvar("MODBUS_HOST_PORT").Int()
	modbusUnitID        = kingpin.Flag("modbus-unit-id", "ModBus unit id of modbus").Default("3").OverrideDefaultFromEnvar("MODBUS_UNIT_ID").Int()

	bigqueryEnable    = kingpin.Flag("bigquery-enable", "Toggle to enable or disable bigquery integration").Default("true").OverrideDefaultFromEnvar("BQ_ENABLE").Bool()
	bigqueryInit      = kingpin.Flag("bigquery-init", "Toggle to enable bigquery table initialization").Default("true").OverrideDefaultFromEnvar("BQ_INIT").Bool()
	bigqueryProjectID = kingpin.Flag("bigquery-project-id", "Google Cloud project id that contains the BigQuery dataset").Envar("BQ_PROJECT_ID").Required().String()
	bigqueryDataset   = kingpin.Flag("bigquery-dataset", "Name of the BigQuery dataset").Envar("BQ_DATASET").Required().String()
	bigqueryTable     = kingpin.Flag("bigquery-table", "Name of the BigQuery table").Envar("BQ_TABLE").Required().String()

	configPath                   = kingpin.Flag("config-path", "Path to the config.yaml file").Default("/configs/config.yaml").OverrideDefaultFromEnvar("CONFIG_PATH").String()
	measurementFilePath          = kingpin.Flag("state-file-path", "Path to file with state.").Default("/configs/last-measurement.json").OverrideDefaultFromEnvar("MEASUREMENT_FILE_PATH").String()
	measurementFileConfigMapName = kingpin.Flag("state-file-configmap-name", "Name of the configmap with state file.").Default("jarvis-modbus-exporter").OverrideDefaultFromEnvar("MEASUREMENT_FILE_CONFIG_MAP_NAME").String()
)

func main() {

	// parse command line parameters
	kingpin.Parse()

	// init log format from envvar ESTAFETTE_LOG_FORMAT
	foundation.InitLoggingFromEnv(foundation.NewApplicationInfo(appgroup, app, version, branch, revision, buildDate))

	// create context to cancel commands on sigterm
	ctx := foundation.InitCancellationContext(context.Background())

	// bootstrap
	configClient, err := NewConfigClient()
	if err != nil {
		log.Fatal().Err(err).Msg("Failed creating configClient")
	}

	bigqueryClient, err := NewBigQueryClient(ctx, *bigqueryProjectID, *bigqueryEnable)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed creating bigqueryClient")
	}

	kubeClientConfig, err := rest.InClusterConfig()
	if err != nil {
		log.Fatal().Err(err).Msg("Failed creating kubeClientConfig")
	}

	kubeClientset, err := kubernetes.NewForConfig(kubeClientConfig)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed creating kubeClienset")
	}

	stateClient, err := NewStateClient(kubeClientset, *measurementFilePath, *measurementFileConfigMapName)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed creating stateClient")
	}

	modbusClient, err := NewModbusClient(*modbusHostIPAddress, *modbusHostPort, *modbusUnitID)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed creating modbusClient")
	}

	exporterService, err := NewExporterService(configClient, bigqueryClient, stateClient, modbusClient)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed creating exporterService")
	}

	// run exporter
	err = exporterService.Run(ctx, *bigqueryInit, *bigqueryDataset, *bigqueryTable)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed running exporter")
	}

}
