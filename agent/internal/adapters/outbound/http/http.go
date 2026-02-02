package http

import (
	"context"
	"io"
	"log/slog"
	"net/http"
	"strings"

	job "github.com/guggr/guggr/gen/proto/go/job"
	jobresult "github.com/guggr/guggr/gen/proto/go/result"
	types "github.com/guggr/guggr/gen/proto/go/result/types"
	"google.golang.org/protobuf/types/known/timestamppb"
)

type HTTPAdapter struct {
	client *http.Client
}

func NewAdapter() *HTTPAdapter {
	return &HTTPAdapter{client: &http.Client{}}
}

func (a *HTTPAdapter) Execute(ctx context.Context, j *job.Job) (jobresult.JobResult, error) {
	config := j.GetHttp()

	slog.Info("executing http check", "jobid", j.GetId(), "url", config.Url)

	resp, err := a.client.Get(config.Url)
	if err != nil {
		return jobresult.JobResult{}, err
	}
	defer resp.Body.Close()

	payload, err := io.ReadAll(resp.Body)

	slog.Info("success for http job", "jobid", j.GetId(), "url", config.Url)
	return jobresult.JobResult{
		Id: j.GetId(),
		Timestamp: &timestamppb.Timestamp{
			Seconds: timestamppb.Now().GetSeconds(),
		},
		Http: &types.HttpJobResult{
			Reachable:  true,
			IpAddress:  []byte(strings.Split(resp.Request.RemoteAddr, ":")[0]),
			StatusCode: int32(resp.StatusCode),
			Payload:    payload,
		},
	}, nil
}
