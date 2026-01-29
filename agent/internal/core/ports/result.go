package ports

import (
	"context"

	jobresult "github.com/guggr/guggr/gen/proto/go/result"
)

type ResultPort interface {
	PublishResult(ctx context.Context, result *jobresult.JobResult) error
}
