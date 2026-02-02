package ping

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	job "github.com/guggr/guggr/gen/proto/go/job"
	jobresult "github.com/guggr/guggr/gen/proto/go/result"
	types "github.com/guggr/guggr/gen/proto/go/result/types"
	probing "github.com/prometheus-community/pro-bing"
	"google.golang.org/protobuf/types/known/durationpb"
)

type PingAdapter struct {
}

func NewAdapter() *PingAdapter {
	return &PingAdapter{}
}

func (a *PingAdapter) Execute(ctx context.Context, j *job.Job) (jobresult.JobResult, error) {
	config := j.GetPing()

	pinger, err := probing.NewPinger(config.Host)
	if err != nil {
		return jobresult.JobResult{}, fmt.Errorf("error creating pinger for host %s: %w", config.Host, err)
	}

	// TODO put this into the JobDetails
	pinger.Count = 1
	pinger.Timeout = time.Second * 1

	slog.Info("icmp pinging...", "job", j.GetId(), "host", config.Host)

	err = pinger.Run()
	if err != nil {
		return jobresult.JobResult{}, fmt.Errorf("icmp job with id %s failed with error: %w", j.GetId(), err)
	}

	stats := pinger.Statistics()
	if stats.PacketsRecv == 0 {
		return jobresult.JobResult{}, fmt.Errorf("icmp job with id %s failed since host is unreachable", config.Host)
	}

	slog.Info("success for icmp job", "jobid", j.GetId(), "host", config.Host)
	return jobresult.JobResult{
		Id: j.GetId(),
		Ping: &types.PingJobResult{
			Reachable: true,
			IpAddress: []byte(config.Host),
			Latency: &durationpb.Duration{
				Nanos: int32(stats.MinRtt.Nanoseconds()),
			},
		},
	}, err
}
