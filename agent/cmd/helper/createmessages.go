package main

import (
	"context"
	"log"
	"time"

	"github.com/gogo/protobuf/proto"
	job "github.com/guggr/guggr/gen/proto/go/job"
	types "github.com/guggr/guggr/gen/proto/go/job/types"
	amqp "github.com/rabbitmq/amqp091-go"
)

func main() {
	conn, err := amqp.Dial("amqp://guggr:guggr@localhost:5672")
	failOnError(err, "Failed to connect to RabbitMQ")
	defer conn.Close()

	// Create channel
	ch, err := conn.Channel()
	failOnError(err, "Failed to open a channel")
	defer ch.Close()

	// For sending a queue is needed
	q, err := ch.QueueDeclare(
		"jobs", // name
		false,  // durable
		false,  // delete when unused
		false,  // exclusive
		false,  // no wait
		nil,    // arguments
	)
	failOnError(err, "Failed to declare a queue")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	pingJob := &job.Job{
		Id: "31a6931a-fe99-468c-b7fb-fb43ab205e05",
		Ping: &types.PingJobType{
			Host: "8.8.8.8",
		},
	}

	httpJob := &job.Job{
		Id: "a66d160e-bb3c-4b6e-9c7e-814956ffbd62",
		Http: &types.HttpJobType{
			Url: "https://google.com",
		},
	}

	jobs := []job.Job{*pingJob, *httpJob}

	for _, job := range jobs {
		out, err := proto.Marshal(&job)
		failOnError(err, "Failed to marshal job")

		body := out
		err = ch.PublishWithContext(ctx,
			"",
			q.Name,
			false,
			false,
			amqp.Publishing{
				ContentType: "text/plain",
				Body:        []byte(body),
			},
		)

		failOnError(err, "Failed to publish a message")
		log.Printf(" [x] Sent %s\n", body)
	}

}

func failOnError(err error, msg string) {
	if err != nil {
		log.Panicf("%s: %s", msg, err)
	}
}
