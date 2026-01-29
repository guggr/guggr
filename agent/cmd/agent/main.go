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
	"github.com/guggr/guggr-agent/internal/core/services"

	amqp "github.com/rabbitmq/amqp091-go"
)

func main() {

	// Setup logging
	logger := slog.New(slog.NewJSONHandler(os.Stdout, nil))
	slog.SetDefault(logger)

	rabbitmqUser := os.Getenv("RABBITMQ_USER")
	rabbitmqPass := os.Getenv("RABBITMQ_PASS")
	rabbitmqHost := os.Getenv("RABBITMQ_HOST")
	rabbitmqPort := os.Getenv("RABBITMQ_PORT")

	connectionString := fmt.Sprintf("amqp://%s:%s@%s:%s/", rabbitmqUser, rabbitmqPass, rabbitmqHost, rabbitmqPort)
	conn, err := amqp.Dial(connectionString)
	if err != nil {
		slog.Error("error connecting to rabbitmq", "error", err)
	}
	defer conn.Close()

	// Outbound adapter
	httpAdapter := http.NewAdapter()
	pingAdapter := ping.NewAdapter()
	rabbitmqPublisherAdapter, err := outboundRabbit.NewRabbitMQPublisher(conn, "jobresults")
	if err != nil {
		slog.Error("error initializing rabbitmq publisher", "error", err)
	}

	// Core service
	jobService := services.NewJobService(httpAdapter, pingAdapter, rabbitmqPublisherAdapter)

	// Inbound adapter
	rabbitmqJobsAdapter, err := inboundRabbit.NewRabbitMQAdapter(
		conn,
		"jobs",
		jobService,
	)

	if err != nil {
		slog.Error("error initializing rabbitmq adapter", "error", err)
	}

	rabbitmqJobsAdapter.Start(10 * time.Second)

}
