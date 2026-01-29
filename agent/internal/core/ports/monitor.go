package ports

import (
	"context"

	job "github.com/guggr/guggr/gen/proto/go/job"
)

type MonitorPort interface {
	Execute(ctx context.Context, job *job.Job) error
}
