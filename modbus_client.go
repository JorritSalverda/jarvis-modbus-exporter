package main

import (
	"context"
	"encoding/binary"
	"fmt"
	"time"

	contractsv1 "github.com/JorritSalverda/jarvis-contracts-golang/contracts/v1"
	"github.com/goburrow/modbus"
	"github.com/google/uuid"
	"github.com/rs/zerolog/log"
)

// ModbusClient is the interface for connecting to a modbus device via ethernet
//go:generate mockgen -package=main -destination ./modbus_client_mock.go -source=modbus_client.go
type ModbusClient interface {
	GetMeasurement(ctx context.Context, config Config, lastMeasurement *contractsv1.Measurement) (measurement contractsv1.Measurement, err error)
	GetSample(ctx context.Context, config Config, sampleConfig ConfigSample, modbusClient modbus.Client) (sample contractsv1.Sample, err error)
}

// NewModbusClient returns new ModbusClient
func NewModbusClient(host string, port int, unitID int) (ModbusClient, error) {
	if host == "" {
		return nil, fmt.Errorf("Please set the ip address of your modbus device on your local network")
	}
	if port != 502 && (port < 49152 || port > 65535) {
		return nil, fmt.Errorf("Please set the modbus port of your modbus device on your local network to its default 502, or anywhere between 49152 and 65535 if changed in the installer menu")
	}

	return &modbusClient{
		host:   host,
		port:   port,
		unitID: unitID,
	}, nil
}

type modbusClient struct {
	host   string
	port   int
	unitID int
}

func (c *modbusClient) GetMeasurement(ctx context.Context, config Config, lastMeasurement *contractsv1.Measurement) (measurement contractsv1.Measurement, err error) {

	// Modbus TCP
	handler := modbus.NewTCPClientHandler(fmt.Sprintf("%v:%v", c.host, c.port))
	handler.Timeout = 20 * time.Second
	handler.SlaveId = byte(c.unitID)
	// Connect manually so that multiple requests are handled in one connection session
	err = handler.Connect()
	if err != nil {
		return
	}
	defer handler.Close()
	client := modbus.NewClient(handler)

	measurement = contractsv1.Measurement{
		ID:             uuid.New().String(),
		Source:         "jarvis-modbus-exporter",
		Location:       config.Location,
		Samples:        []*contractsv1.Sample{},
		MeasuredAtTime: time.Now().UTC(),
	}

	for _, sc := range config.SampleConfigs {
		sample, sampleErr := c.GetSample(ctx, config, sc, client)
		if sampleErr != nil {
			return measurement, sampleErr
		}
		measurement.Samples = append(measurement.Samples, &sample)
	}

	if lastMeasurement != nil {
		measurement.Samples = c.sanitizeSamples(measurement.Samples, lastMeasurement.Samples)
	}

	return
}

func (c *modbusClient) GetSample(ctx context.Context, config Config, sampleConfig ConfigSample, modbusClient modbus.Client) (sample contractsv1.Sample, err error) {

	var sampleBytes []byte

	switch sampleConfig.RegisterType {
	case RegisterTypeInput:
		sampleBytes, err = modbusClient.ReadInputRegisters(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	case RegisterTypeHolding:
		sampleBytes, err = modbusClient.ReadHoldingRegisters(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	case RegisterTypeDiscrete:
		sampleBytes, err = modbusClient.ReadDiscreteInputs(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	case RegisterTypeCoil:
		sampleBytes, err = modbusClient.ReadCoils(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	}

	// init sample from config
	sample = contractsv1.Sample{
		EntityType: sampleConfig.EntityType,
		EntityName: sampleConfig.EntityName,
		SampleType: sampleConfig.SampleType,
		SampleName: sampleConfig.SampleName,
		MetricType: sampleConfig.MetricType,
	}

	// convert sample to float and correct
	sampleValue := binary.BigEndian.Uint64(sampleBytes)
	sampleValueAsFloat64 := float64(sampleValue) * sampleConfig.ValueMultiplier

	sample.Value = sampleValueAsFloat64

	return
}

func (c *modbusClient) sanitizeSamples(currentSamples, lastSamples []*contractsv1.Sample) (sanitizeSamples []*contractsv1.Sample) {

	sanitizeSamples = []*contractsv1.Sample{}
	for _, cs := range currentSamples {
		// check if there's a corresponding sample in lastSamples and see if the difference with it's value isn't too large
		sanitize := false
		for _, ls := range lastSamples {
			if cs.EntityType == ls.EntityType &&
				cs.EntityName == ls.EntityName &&
				cs.SampleType == ls.SampleType &&
				cs.SampleName == ls.SampleName &&
				cs.MetricType == cs.MetricType {
				if cs.MetricType == contractsv1.MetricType_METRIC_TYPE_COUNTER && cs.Value < ls.Value {
					sanitize = true
					log.Warn().Msgf("Value for %v is less than the last sampled value %v, keeping previous value instead", cs, ls.Value)
					sanitizeSamples = append(sanitizeSamples, ls)
				} else if cs.MetricType == contractsv1.MetricType_METRIC_TYPE_COUNTER && cs.Value/ls.Value > 1.1 {
					sanitize = true
					log.Warn().Msgf("Value for %v is more than 10 percent larger than the last sampled value %v, keeping previous value instead", cs, ls.Value)
					sanitizeSamples = append(sanitizeSamples, ls)
				}

				break
			}
		}
		if !sanitize {
			sanitizeSamples = append(sanitizeSamples, cs)
		}
	}

	return
}
