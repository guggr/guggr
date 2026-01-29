package ping

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	job "github.com/guggr/guggr/gen/proto/go/job"
	probing "github.com/prometheus-community/pro-bing"
)

type Adapter struct {
}

func NewAdapter() *Adapter {
	return &Adapter{}
}

func (a *Adapter) Execute(ctx context.Context, j *job.Job) error {
	config := j.GetPing()

	pinger, err := probing.NewPinger(config.Host)
	if err != nil {
		return fmt.Errorf("error creating pinger for host %s: %w", config.Host, err)
	}

	// TODO put this into the JobDetails
	pinger.Count = 1
	pinger.Timeout = time.Second * 1

	slog.Info("icmp pinging...", "job", j.GetId(), "host", config.Host)

	err = pinger.Run()
	if err != nil {
		return fmt.Errorf("icmp job with id %s failed with error: %w", j.GetId(), err)
	}

	stats := pinger.Statistics()
	if stats.PacketsRecv == 0 {
		return fmt.Errorf("icmp job with id %s failed since host is unreachable", config.Host)
	}

	slog.Info("success for icmp job", "jobid", j.GetId(), "host", config.Host)
	return nil
}
