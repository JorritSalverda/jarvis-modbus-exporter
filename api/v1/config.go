package api

import (
	contractsv1 "github.com/JorritSalverda/jarvis-contracts-golang/contracts/v1"
)

type Config struct {
	Location      string         `yaml:"location"`
	SampleConfigs []ConfigSample `yaml:"sampleConfigs"`
}

type ConfigSample struct {
	// default jarvis config for sample
	Name             string                       `yaml:"name"`
	DisplayName      string                       `yaml:"displayName"`
	AggregationLevel contractsv1.AggregationLevel `yaml:"aggregationLevel"`
	MetricType       contractsv1.MetricType       `yaml:"metricType"`
	SampleType       contractsv1.SampleType       `yaml:"sampleType"`
	SampleUnit       contractsv1.SampleUnit       `yaml:"sampleUnit"`

	// modbus specific config for sample
	ValueMultiplier  float64      `yaml:"valueMultiplier"`
	RegisterType     RegisterType `yaml:"registerType"`
	RegisterAddress  uint16       `yaml:"registerAddress"`
	RegisterQuantity uint16       `yaml:"registerQuantity"`
}

type RegisterType string

const (
	RegisterTypeInput    RegisterType = "input"
	RegisterTypeHolding  RegisterType = "holding"
	RegisterTypeDiscrete RegisterType = "discrete"
	RegisterTypeCoil     RegisterType = "coil"
)
