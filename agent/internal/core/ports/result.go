package ports

import (
	"context"

	jobresult "github.com/guggr/guggr/gen/pkg/result"
)

type ResultPort interface {
	PublishResult(ctx context.Context, result *jobresult.JobResult) error
}
