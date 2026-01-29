package services

import (
	"context"
	"errors"

	"github.com/guggr/guggr-agent/internal/adapters/outbound/http"
	"github.com/guggr/guggr-agent/internal/adapters/outbound/ping"
	"github.com/guggr/guggr-agent/internal/core/ports"
	job "github.com/guggr/guggr/gen/proto/go/job"
)

type JobService struct {
	httpAdapter ports.MonitorPort
	pingAdapter ports.MonitorPort
}

func NewJobService(httpAdapter *http.Adapter, pingAdapter *ping.Adapter) JobService {
	return JobService{
		httpAdapter: httpAdapter,
		pingAdapter: pingAdapter,
	}
}

func (s *JobService) ProcessJob(ctx context.Context, j *job.Job) error {

	if j.GetHttp() != nil {
		return s.httpAdapter.Execute(ctx, j)
	}

	if j.GetPing() != nil {
		return s.pingAdapter.Execute(ctx, j)
	}

	return errors.New("unknown job type supplied")
}
