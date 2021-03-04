// Code generated by MockGen. DO NOT EDIT.
// Source: bigquery_client.go

// Package main is a generated GoMock package.
package main

import (
	context "context"
	contracts "github.com/JorritSalverda/jarvis-contracts-golang/contracts/v1"
	gomock "github.com/golang/mock/gomock"
	reflect "reflect"
)

// MockBigQueryClient is a mock of BigQueryClient interface
type MockBigQueryClient struct {
	ctrl     *gomock.Controller
	recorder *MockBigQueryClientMockRecorder
}

// MockBigQueryClientMockRecorder is the mock recorder for MockBigQueryClient
type MockBigQueryClientMockRecorder struct {
	mock *MockBigQueryClient
}

// NewMockBigQueryClient creates a new mock instance
func NewMockBigQueryClient(ctrl *gomock.Controller) *MockBigQueryClient {
	mock := &MockBigQueryClient{ctrl: ctrl}
	mock.recorder = &MockBigQueryClientMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use
func (m *MockBigQueryClient) EXPECT() *MockBigQueryClientMockRecorder {
	return m.recorder
}

// CheckIfDatasetExists mocks base method
func (m *MockBigQueryClient) CheckIfDatasetExists(ctx context.Context, dataset string) bool {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "CheckIfDatasetExists", ctx, dataset)
	ret0, _ := ret[0].(bool)
	return ret0
}

// CheckIfDatasetExists indicates an expected call of CheckIfDatasetExists
func (mr *MockBigQueryClientMockRecorder) CheckIfDatasetExists(ctx, dataset interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "CheckIfDatasetExists", reflect.TypeOf((*MockBigQueryClient)(nil).CheckIfDatasetExists), ctx, dataset)
}

// CheckIfTableExists mocks base method
func (m *MockBigQueryClient) CheckIfTableExists(ctx context.Context, dataset, table string) bool {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "CheckIfTableExists", ctx, dataset, table)
	ret0, _ := ret[0].(bool)
	return ret0
}

// CheckIfTableExists indicates an expected call of CheckIfTableExists
func (mr *MockBigQueryClientMockRecorder) CheckIfTableExists(ctx, dataset, table interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "CheckIfTableExists", reflect.TypeOf((*MockBigQueryClient)(nil).CheckIfTableExists), ctx, dataset, table)
}

// CreateTable mocks base method
func (m *MockBigQueryClient) CreateTable(ctx context.Context, dataset, table string, typeForSchema interface{}, partitionField string, waitReady bool) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "CreateTable", ctx, dataset, table, typeForSchema, partitionField, waitReady)
	ret0, _ := ret[0].(error)
	return ret0
}

// CreateTable indicates an expected call of CreateTable
func (mr *MockBigQueryClientMockRecorder) CreateTable(ctx, dataset, table, typeForSchema, partitionField, waitReady interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "CreateTable", reflect.TypeOf((*MockBigQueryClient)(nil).CreateTable), ctx, dataset, table, typeForSchema, partitionField, waitReady)
}

// UpdateTableSchema mocks base method
func (m *MockBigQueryClient) UpdateTableSchema(ctx context.Context, dataset, table string, typeForSchema interface{}) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "UpdateTableSchema", ctx, dataset, table, typeForSchema)
	ret0, _ := ret[0].(error)
	return ret0
}

// UpdateTableSchema indicates an expected call of UpdateTableSchema
func (mr *MockBigQueryClientMockRecorder) UpdateTableSchema(ctx, dataset, table, typeForSchema interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "UpdateTableSchema", reflect.TypeOf((*MockBigQueryClient)(nil).UpdateTableSchema), ctx, dataset, table, typeForSchema)
}

// DeleteTable mocks base method
func (m *MockBigQueryClient) DeleteTable(ctx context.Context, dataset, table string) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "DeleteTable", ctx, dataset, table)
	ret0, _ := ret[0].(error)
	return ret0
}

// DeleteTable indicates an expected call of DeleteTable
func (mr *MockBigQueryClientMockRecorder) DeleteTable(ctx, dataset, table interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "DeleteTable", reflect.TypeOf((*MockBigQueryClient)(nil).DeleteTable), ctx, dataset, table)
}

// InsertMeasurement mocks base method
func (m *MockBigQueryClient) InsertMeasurement(ctx context.Context, dataset, table string, measurement contracts.Measurement) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "InsertMeasurement", ctx, dataset, table, measurement)
	ret0, _ := ret[0].(error)
	return ret0
}

// InsertMeasurement indicates an expected call of InsertMeasurement
func (mr *MockBigQueryClientMockRecorder) InsertMeasurement(ctx, dataset, table, measurement interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "InsertMeasurement", reflect.TypeOf((*MockBigQueryClient)(nil).InsertMeasurement), ctx, dataset, table, measurement)
}

// InitBigqueryTable mocks base method
func (m *MockBigQueryClient) InitBigqueryTable(ctx context.Context, dataset, table string) error {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "InitBigqueryTable", ctx, dataset, table)
	ret0, _ := ret[0].(error)
	return ret0
}

// InitBigqueryTable indicates an expected call of InitBigqueryTable
func (mr *MockBigQueryClientMockRecorder) InitBigqueryTable(ctx, dataset, table interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "InitBigqueryTable", reflect.TypeOf((*MockBigQueryClient)(nil).InitBigqueryTable), ctx, dataset, table)
}
