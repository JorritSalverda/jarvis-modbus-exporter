package contracts

import "time"

type Measurement struct {
	Source         string
	Location       string
	Samples        []*Sample
	MeasuredAtTime time.Time
}
