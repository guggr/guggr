package rabbitmq

import (
	"context"
	"fmt"

	"github.com/gogo/protobuf/proto"
	jobresult "github.com/guggr/guggr/gen/proto/go/result"
	amqp "github.com/rabbitmq/amqp091-go"
)

type options struct {
	conn      *amqp.Connection
	queueName *string
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

type RabbitMQPublisher struct {
	conn    *amqp.Connection
	channel *amqp.Channel
	queue   string
}

func NewRabbitMQPublisher(opts ...Option) (*RabbitMQPublisher, error) {
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

	return &RabbitMQPublisher{
		channel: ch,
		queue:   *options.queueName,
	}, nil
}

func (p *RabbitMQPublisher) PublishResult(ctx context.Context, jobresult *jobresult.JobResult) error {
	q, err := p.channel.QueueDeclare(
		p.queue, // name
		true,    // durable
		false,   // delete when unused
		false,   // exclusive
		false,   // no wait
		amqp.Table{
			"x-queue-type": amqp.QueueTypeQuorum,
		}, // arguments
	)

	if err != nil {
		return fmt.Errorf("error declaring jobresults queue: %v", err)
	}

	body, err := proto.Marshal(jobresult)
	if err != nil {
		return fmt.Errorf("error marshaling job result: %v", err)
	}

	err = p.channel.PublishWithContext(
		ctx,
		"",
		q.Name,
		false,
		false,
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        []byte(body),
		},
	)
	if err != nil {
		return fmt.Errorf("error publishing job result: %v", err)
	}

	return nil

}
