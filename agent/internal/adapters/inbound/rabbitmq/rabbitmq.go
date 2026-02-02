package rabbitmq

import (
	"context"
	"log/slog"
	"time"

	"github.com/guggr/guggr-agent/internal/core/service"
	job "github.com/guggr/guggr/gen/pkg/job"
	amqp "github.com/rabbitmq/amqp091-go"
	"google.golang.org/protobuf/proto"
)

type options struct {
	conn      *amqp.Connection
	queueName *string
	service   *service.JobService
}

type Option func(options *options) error

func WithConnection(conn *amqp.Connection) Option {
	return func(options *options) error {
		options.conn = conn
		return nil
	}
}

func WithQueueName(queueName *string) Option {
	return func(options *options) error {
		options.queueName = queueName
		return nil
	}
}

func WithService(service *service.JobService) Option {
	return func(options *options) error {
		options.service = service
		return nil
	}
}

type RabbitMQAdapter struct {
	service   service.JobService
	conn      *amqp.Connection
	channel   *amqp.Channel
	queueName string
}

func NewRabbitMQAdapter(opts ...Option) (*RabbitMQAdapter, error) {
	var options options
	for _, opt := range opts {
		err := opt(&options)
		if err != nil {
			return nil, err
		}
	}

	ch, err := options.conn.Channel()
	if err != nil {
		return nil, err
	}

	return &RabbitMQAdapter{
		channel:   ch,
		queueName: *options.queueName,
		service:   *options.service,
	}, nil
}

func (a *RabbitMQAdapter) Start() {
	msgs, err := a.channel.Consume(
		a.queueName,
		"", // consumer name
		false,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		slog.Error("error getting messages from rabbitmq", "error", err)
	}

	var infinite chan struct{}

	go func() {
		for msg := range msgs {
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

				err := a.service.ProcessJob(ctx, &j)
				if err != nil {
					slog.Error("error executing job", "jobid", j.GetId(), "error", err)
					return
				} else {
					msg.Ack(false)
				}
			}(msg.Body)
		}
	}()

	<-infinite
}
