package main

import (
	"context"
	"testing"

	contractsv1 "github.com/JorritSalverda/jarvis-contracts-golang/contracts/v1"
	"github.com/stretchr/testify/assert"
)

func TestReadConfigFromFile(t *testing.T) {

	t.Run("ReturnsConfig", func(t *testing.T) {

		ctx := context.Background()
		client, _ := NewConfigClient()

		// act
		config, err := client.ReadConfigFromFile(ctx, "./test-config.yaml")

		assert.Nil(t, err)
		assert.Equal(t, "My Home", config.Location)
		assert.Equal(t, 1, len(config.SampleConfigs))
		assert.Equal(t, contractsv1.EntityType_ENTITY_TYPE_DEVICE, config.SampleConfigs[0].EntityType)
		assert.Equal(t, "Sunny TriPower 8.0", config.SampleConfigs[0].EntityName)
		assert.Equal(t, contractsv1.SampleType_SAMPLE_TYPE_ELECTRICITY_PRODUCTION, config.SampleConfigs[0].SampleType)
		assert.Equal(t, "Totaal opgewekt", config.SampleConfigs[0].SampleName)
		assert.Equal(t, contractsv1.MetricType_METRIC_TYPE_COUNTER, config.SampleConfigs[0].MetricType)
	})
}