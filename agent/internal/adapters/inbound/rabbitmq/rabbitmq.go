package rabbitmq

import (
	"context"
	"log/slog"
	"time"

	"github.com/guggr/guggr-agent/internal/core/services"
	job "github.com/guggr/guggr/gen/proto/go/job"
	amqp "github.com/rabbitmq/amqp091-go"
	"google.golang.org/protobuf/proto"
)

type RabbitMQAdapter struct {
	service services.JobService
	conn    *amqp.Connection
	channel *amqp.Channel
	queue   string
}

func NewRabbitMQAdapter(conn *amqp.Connection, queueName string, service services.JobService) (*RabbitMQAdapter, error) {
	ch, err := conn.Channel()
	if err != nil {
		return nil, err
	}

	return &RabbitMQAdapter{
		channel: ch,
		queue:   queueName,
		service: service,
	}, nil
}

func (a *RabbitMQAdapter) Start(interval time.Duration) {
	ticker := time.NewTicker(interval)
	for range ticker.C {
		for {
			// Get message from queue
			msg, ok, err := a.channel.Get(a.queue, true)
			if err != nil {
				slog.Error("error getting messages from rabbitmq", "error", err)
			}
			if !ok {
				// No message in Queue
				break
			}

			// create a go routine for every job
			go func(d []byte) {
				// Unmarshal protobuf
				var j job.Job
				if err := proto.Unmarshal(msg.Body, &j); err != nil {
					slog.Error("error decoding job from rabbitmq", "error", err)
					return
				}

				// Send job to core
				ctx, cancel := context.WithTimeout(context.Background(), 15*time.Second)
				defer cancel()

				if err := a.service.ProcessJob(ctx, &j); err != nil {
					slog.Error("error executing job", "jobid", j.GetId(), "error", err)
				}
			}(msg.Body)
		}
	}
}
