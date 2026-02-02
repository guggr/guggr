package main

import (
	"fmt"
	"log/slog"
	"os"
	"time"

	inboundRabbit "github.com/guggr/guggr-agent/internal/adapters/inbound/rabbitmq"
	"github.com/guggr/guggr-agent/internal/adapters/outbound/http"
	"github.com/guggr/guggr-agent/internal/adapters/outbound/ping"
	outboundRabbit "github.com/guggr/guggr-agent/internal/adapters/outbound/rabbitmq"
	"github.com/guggr/guggr-agent/internal/core/service"

	amqp "github.com/rabbitmq/amqp091-go"
)

func main() {

	// Setup logging
	logger := slog.New(slog.NewJSONHandler(os.Stdout, nil))
	slog.SetDefault(logger)

	rabbitmqUser := getEnvOrDefault("RABBITMQ_USER", "guggr")
	rabbitmqPass := getEnvOrDefault("RABBITMQ_PASS", "guggr")
	rabbitmqHost := getEnvOrDefault("RABBITMQ_HOST", "localhost")
	rabbitmqPort := getEnvOrDefault("RABBITMQ_PORT", "5672")

	connectionString := fmt.Sprintf("amqp://%s:%s@%s:%s/", rabbitmqUser, rabbitmqPass, rabbitmqHost, rabbitmqPort)
	var conn *amqp.Connection
	conn, err := amqp.Dial(connectionString)
	var connectionRetries = 0
	for err != nil {
		slog.Error("error connecting to rabbitmq", "error", err)
		connectionRetries++
		if connectionRetries == 5 {
			slog.Error("could not connect to rabbitmq after 5 retries, please check your rabbitmq host")
			os.Exit(1)
		}
		time.Sleep(10 * time.Second)
	}
	defer conn.Close()

	// Outbound adapter
	httpAdapter := http.NewAdapter()
	pingAdapter := ping.NewAdapter()
	jobResultQueueName := getEnvOrDefault("RABBITMQ_JOB_RESULT_QUEUE_NAME", "jobresults")
	rabbitmqPublisherAdapter, err := outboundRabbit.NewRabbitMQPublisher(outboundRabbit.WithConnection(conn), outboundRabbit.WithQueueName(&jobResultQueueName))
	if err != nil {
		slog.Error("error initializing rabbitmq publisher", "error", err)
	}

	// Core service
	jobService, err := service.NewJobService(service.WithHttpAdapter(httpAdapter), service.WithPingAdapter(pingAdapter), service.WithPublisherAdapter(rabbitmqPublisherAdapter))
	if err != nil {
		slog.Error("error initializing job service")
	}

	// Inbound adapter
	jobQueueName := getEnvOrDefault("RABBITMQ_JOB_QUEUE_NAME", "jobs")
	rabbitmqJobsAdapter, err := inboundRabbit.NewRabbitMQAdapter(
		inboundRabbit.WithConnection(conn),
		inboundRabbit.WithQueueName(&jobQueueName),
		inboundRabbit.WithService(jobService),
	)

	if err != nil {
		slog.Error("error initializing rabbitmq adapter", "error", err)
	}

	rabbitmqJobsAdapter.Start()

}

func getEnvOrDefault(envName string, defaultValue string) string {
	if value, exists := os.LookupEnv(envName); exists {
		return value
	}
	return defaultValue
}
