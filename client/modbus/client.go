package modbus

import (
	"encoding/binary"
	"fmt"
	"log"
	"os"
	"time"

	contractsv1 "github.com/JorritSalverda/jarvis-contracts-golang/contracts/v1"
	apiv1 "github.com/JorritSalverda/jarvis-modbus-exporter/api/v1"
	"github.com/goburrow/modbus"
)

// Client is the interface for connecting to a modbus device via ethernet
type Client interface {
	GetMeasurement(config apiv1.Config) (measurement contractsv1.Measurement, err error)
	GetSample(config apiv1.Config, sampleConfig apiv1.ConfigSample, modbusClient modbus.Client) (sample contractsv1.Sample, err error)
}

// NewClient returns new modbus.Client
func NewClient(host string, port int, unitID int) (Client, error) {
	if host == "" {
		return nil, fmt.Errorf("Please set the ip address of your modbus device on your local network")
	}
	if port != 502 && (port < 49152 || port > 65535) {
		return nil, fmt.Errorf("Please set the modbus port of your modbus device on your local network to its default 502, or anywhere between 49152 and 65535 if changed in the installer menu")
	}

	return &client{
		host:   host,
		port:   port,
		unitID: unitID,
	}, nil
}

type client struct {
	host   string
	port   int
	unitID int
}

func (c *client) GetMeasurement(config apiv1.Config) (measurement contractsv1.Measurement, err error) {

	// Modbus TCP
	handler := modbus.NewTCPClientHandler(fmt.Sprintf("%v:%v", c.host, c.port))
	handler.Timeout = 20 * time.Second
	handler.SlaveId = 0x3
	handler.Logger = log.New(os.Stdout, "test: ", log.LstdFlags)
	// Connect manually so that multiple requests are handled in one connection session
	err = handler.Connect()
	if err != nil {
		return
	}
	defer handler.Close()
	client := modbus.NewClient(handler)

	measurement = contractsv1.Measurement{
		Source:   "jarvis-modbus-exporter",
		Location: config.Location,
		Samples:  []*contractsv1.Sample{},
	}

	for _, sc := range config.SampleConfigs {
		sample, sampleErr := c.GetSample(config, sc, client)
		if sampleErr != nil {
			return measurement, sampleErr
		}
		measurement.Samples = append(measurement.Samples, &sample)
	}

	return
}

func (c *client) GetSample(config apiv1.Config, sampleConfig apiv1.ConfigSample, modbusClient modbus.Client) (sample contractsv1.Sample, err error) {

	var sampleBytes []byte

	switch sampleConfig.RegisterType {
	case apiv1.RegisterTypeInput:
		sampleBytes, err = modbusClient.ReadInputRegisters(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	case apiv1.RegisterTypeHolding:
		sampleBytes, err = modbusClient.ReadHoldingRegisters(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	case apiv1.RegisterTypeDiscrete:
		sampleBytes, err = modbusClient.ReadDiscreteInputs(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	case apiv1.RegisterTypeCoil:
		sampleBytes, err = modbusClient.ReadCoils(sampleConfig.RegisterAddress, sampleConfig.RegisterQuantity)
		if err != nil {
			return
		}
	}

	// init sample from config
	sample = contractsv1.Sample{
		DeviceName:       sampleConfig.DeviceName,
		SampleName:       sampleConfig.SampleName,
		AggregationLevel: sampleConfig.AggregationLevel,
		MetricType:       sampleConfig.MetricType,
		SampleType:       sampleConfig.SampleType,
		SampleUnit:       sampleConfig.SampleUnit,
	}

	// convert sample to float and correct
	sampleValue := binary.BigEndian.Uint64(sampleBytes)
	sampleValueAsFloat64 := float64(sampleValue) * sampleConfig.ValueMultiplier

	sample.Value = sampleValueAsFloat64

	return
}
