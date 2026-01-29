package http

import (
	"context"
	"fmt"
	"log/slog"
	"net/http"

	job "github.com/guggr/guggr/gen/proto/go/job"
)

type Adapter struct {
	client *http.Client
}

func NewAdapter() *Adapter {
	return &Adapter{client: &http.Client{}}
}

func (a *Adapter) Execute(ctx context.Context, j *job.Job) error {
	config := j.GetHttp()

	slog.Info("executing http check", "jobid", j.GetId(), "url", config.Url)

	resp, err := a.client.Get(config.Url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return fmt.Errorf("received http response %d instead of 200", resp.StatusCode)
	}

	slog.Info("success for http job", "jobid", j.GetId(), "url", config.Url)
	return nil
}
