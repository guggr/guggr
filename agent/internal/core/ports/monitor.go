package ports

import (
	"context"

	job "github.com/guggr/guggr/gen/proto/go/job"
	jobresult "github.com/guggr/guggr/gen/proto/go/result"
)

type MonitorPort interface {
	Execute(ctx context.Context, job *job.Job) (jobresult.JobResult, error)
}
