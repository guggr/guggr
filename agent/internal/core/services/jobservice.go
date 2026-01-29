package services

import (
	"context"
	"errors"

	"github.com/guggr/guggr-agent/internal/adapters/outbound/http"
	"github.com/guggr/guggr-agent/internal/adapters/outbound/ping"
	"github.com/guggr/guggr-agent/internal/adapters/outbound/rabbitmq"
	"github.com/guggr/guggr-agent/internal/core/ports"
	job "github.com/guggr/guggr/gen/proto/go/job"
	jobresult "github.com/guggr/guggr/gen/proto/go/result"
)

type JobService struct {
	httpAdapter            ports.MonitorPort
	pingAdapter            ports.MonitorPort
	resultPublisherAdapter ports.ResultPort
}

func NewJobService(httpAdapter *http.Adapter, pingAdapter *ping.Adapter, resultPublisherAdapter *rabbitmq.RabbitMQPublisher) JobService {
	return JobService{
		httpAdapter:            httpAdapter,
		pingAdapter:            pingAdapter,
		resultPublisherAdapter: resultPublisherAdapter,
	}
}

func (s *JobService) ProcessJob(ctx context.Context, j *job.Job) error {
	var result jobresult.JobResult
	var err error

	if j.GetHttp() != nil {
		result, err = s.httpAdapter.Execute(ctx, j)
	} else if j.GetPing() != nil {
		result, err = s.pingAdapter.Execute(ctx, j)
	} else {
		return errors.New("unknown job type supplied")
	}

	if err != nil {
		return err
	}

	return s.resultPublisherAdapter.PublishResult(ctx, &result)
}
