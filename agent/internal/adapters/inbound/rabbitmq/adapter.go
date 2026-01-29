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

func (w *RabbitMQAdapter) Start(interval time.Duration) {
	ticker := time.NewTicker(interval)
	for range ticker.C {
		// Get message from queue
		msg, ok, _ := w.channel.Get(w.queue, true)
		if !ok {
			// No message in Queue
			continue
		}

		// Unmarshal protobuf
		var j job.Job
		if err := proto.Unmarshal(msg.Body, &j); err != nil {
			slog.Error("error decoding job from rabbitmq", "error", err)
			continue
		}

		// Send job to core
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		err := w.service.ProcessJob(ctx, &j)
		if err != nil {
			slog.Error("error executing job", "jobid", j.GetId(), "error", err)
		}
		cancel()
	}
}
