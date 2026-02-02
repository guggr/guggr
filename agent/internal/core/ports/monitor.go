package ports

import (
	"context"

	job "github.com/guggr/guggr/gen/pkg/job"
	jobresult "github.com/guggr/guggr/gen/pkg/result"
)

type MonitorPort interface {
	Execute(ctx context.Context, job *job.Job) (jobresult.JobResult, error)
}
