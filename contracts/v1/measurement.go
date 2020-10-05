package contracts

import (
	"time"

	"github.com/google/uuid"
)

type Measurement struct {
	ID             uuid.UUID
	Source         string
	Location       string
	Samples        []*Sample
	MeasuredAtTime time.Time
}
