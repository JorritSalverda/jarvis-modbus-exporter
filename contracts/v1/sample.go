package contracts

type Sample struct {
	DeviceName       string           `protobuf:"bytes,1,opt,name=device_name,json=deviceName,proto3" json:"device_name,omitempty"`
	SampleName       string           `protobuf:"bytes,2,opt,name=sample_name,json=sampleName,proto3" json:"sample_name,omitempty"`
	AggregationLevel AggregationLevel `protobuf:"varint,3,opt,name=aggregation_level,json=aggregationLevel,proto3,enum=jarvis.contracts.v1.AggregationLevel" json:"aggregation_level,omitempty"`
	MetricType       MetricType       `protobuf:"varint,4,opt,name=metric_type,json=metricType,proto3,enum=jarvis.contracts.v1.MetricType" json:"metric_type,omitempty"`
	Value            float64          `protobuf:"fixed64,5,opt,name=value,proto3" json:"value,omitempty"`
	SampleType       SampleType       `protobuf:"varint,6,opt,name=sample_type,json=sampleType,proto3,enum=jarvis.contracts.v1.SampleType" json:"sample_type,omitempty"`
	SampleUnit       SampleUnit       `protobuf:"varint,7,opt,name=sample_unit,json=sampleUnit,proto3,enum=jarvis.contracts.v1.SampleUnit" json:"sample_unit,omitempty"`
}
