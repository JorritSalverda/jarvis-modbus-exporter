package contracts

type AggregationLevel string

const (
	AggregationLevel_AGGREGATION_LEVEL_INVALID    AggregationLevel = ""
	AggregationLevel_AGGREGATION_LEVEL_METER      AggregationLevel = "AGGREGATION_LEVEL_METER"
	AggregationLevel_AGGREGATION_LEVEL_GROUP      AggregationLevel = "AGGREGATION_LEVEL_GROUP"
	AggregationLevel_AGGREGATION_LEVEL_DEVICE     AggregationLevel = "AGGREGATION_LEVEL_DEVICE"
	AggregationLevel_AGGREGATION_LEVEL_DEVICE_SUB AggregationLevel = "AGGREGATION_LEVEL_DEVICE_SUB"
)
