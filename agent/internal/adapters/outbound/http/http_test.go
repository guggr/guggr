package http

import (
	"context"
	"testing"
	"time"

	"github.com/MarvinJWendt/testza"
	job "github.com/guggr/guggr/gen/pkg/job"
	types "github.com/guggr/guggr/gen/pkg/job/types"
	jobresult "github.com/guggr/guggr/gen/pkg/result"
	resulttypes "github.com/guggr/guggr/gen/pkg/result/types"
	"github.com/jarcoal/httpmock"
	"google.golang.org/protobuf/types/known/timestamppb"
)

func TestExecute(t *testing.T) {
	httpmock.Activate(t)

	httpmock.RegisterResponder("GET", "http://guggr.example", httpmock.NewStringResponder(200, "lorem ipsum"))

	sampleJob := job.Job{
		Id: "vNRUTtYc-RpMu6JAI2TMJ",
		Http: &types.HttpJobType{
			Url: "http://guggr.example",
		},
	}
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	result, err := NewAdapter().Execute(ctx, &sampleJob)
	if err != nil {
		t.Errorf("error testing http execute: %v", err)
	}

	expectedResult := jobresult.JobResult{
		Id: sampleJob.GetId(),
		Timestamp: &timestamppb.Timestamp{
			Seconds: timestamppb.Now().GetSeconds(),
		},
		Http: &resulttypes.HttpJobResult{
			Reachable:  true,
			IpAddress:  []byte(sampleJob.GetHttp().GetUrl()),
			StatusCode: 200,
			Payload:    []byte("lorem ipsum"),
		},
	}

	testza.AssertEqualValues(t, result, expectedResult)
}
