package main

import (
	"context"
	"testing"

	contractsv1 "github.com/JorritSalverda/jarvis-contracts-golang/contracts/v1"
	gomock "github.com/golang/mock/gomock"
	"github.com/stretchr/testify/assert"
)

func TestRun(t *testing.T) {
	t.Run("ReadsConfigFromFile", func(t *testing.T) {

		ctx := context.Background()
		ctrl := gomock.NewController(t)
		defer ctrl.Finish()

		configClient := NewMockConfigClient(ctrl)
		bigqueryClient := NewMockBigQueryClient(ctrl)
		stateClient := NewMockStateClient(ctrl)
		modbusClient := NewMockModbusClient(ctrl)

		bigqueryInit := true
		bigqueryDataset := "dataset"
		bigqueryTable := "table"
		config := Config{}
		measurement := contractsv1.Measurement{}

		service, _ := NewExporterService(configClient, bigqueryClient, stateClient, modbusClient)

		configClient.EXPECT().ReadConfigFromFile(ctx, gomock.Any()).Return(config, nil)
		bigqueryClient.EXPECT().InitBigqueryTable(ctx, bigqueryDataset, bigqueryTable).Return(nil)
		stateClient.EXPECT().ReadState(ctx).Return(nil, nil)
		modbusClient.EXPECT().GetMeasurement(ctx, config, nil).Return(measurement, nil)
		bigqueryClient.EXPECT().InsertMeasurement(ctx, bigqueryDataset, bigqueryTable, measurement).Return(nil)
		stateClient.EXPECT().StoreState(ctx, measurement).Return(nil)

		// act
		err := service.Run(ctx, bigqueryInit, bigqueryDataset, bigqueryTable)

		assert.Nil(t, err)
	})
}
