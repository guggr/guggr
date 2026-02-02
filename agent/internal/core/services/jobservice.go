package services

import (
	"context"
	"errors"

	"github.com/guggr/guggr-agent/internal/core/ports"
	job "github.com/guggr/guggr/gen/proto/go/job"
	jobresult "github.com/guggr/guggr/gen/proto/go/result"
)

type options struct {
	httpAdapter     ports.MonitorPort
	pingAdapter     ports.MonitorPort
	resultPublisher ports.ResultPort
}

type Option func(options *options) error

func WithHttpAdapter(httpAdapter ports.MonitorPort) Option {
	return func(options *options) error {
		options.httpAdapter = httpAdapter
		return nil
	}
}

func WithPingAdapter(pingAdapter ports.MonitorPort) Option {
	return func(options *options) error {
		options.pingAdapter = pingAdapter
		return nil
	}
}

func WithPublisherAdapter(publisherAdapter ports.ResultPort) Option {
	return func(options *options) error {
		options.resultPublisher = publisherAdapter
		return nil
	}
}

type JobService struct {
	httpAdapter            ports.MonitorPort
	pingAdapter            ports.MonitorPort
	resultPublisherAdapter ports.ResultPort
}

func NewJobService(opts ...Option) (*JobService, error) {
	var options options
	for _, opt := range opts {
		err := opt(&options)
		if err != nil {
			return nil, err
		}

	}

	return &JobService{
		httpAdapter:            options.httpAdapter,
		pingAdapter:            options.pingAdapter,
		resultPublisherAdapter: options.resultPublisher,
	}, nil
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
