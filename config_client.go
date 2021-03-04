package main

import (
	"context"
	"io/ioutil"

	"github.com/rs/zerolog/log"
	"gopkg.in/yaml.v2"
)

//go:generate mockgen -package=main -destination ./config_client_mock.go -source=config_client.go
type ConfigClient interface {
	ReadConfigFromFile(ctx context.Context, path string) (config Config, err error)
}

func NewConfigClient() (ConfigClient, error) {
	return &configClient{}, nil
}

type configClient struct {
}

func (c *configClient) ReadConfigFromFile(ctx context.Context, path string) (config Config, err error) {
	log.Debug().Msgf("Reading %v file...", path)

	data, err := ioutil.ReadFile(path)
	if err != nil {
		return config, err
	}

	if err := yaml.UnmarshalStrict(data, &config); err != nil {
		return config, err
	}

	return
}
