# Design

## Architecture

- Scheduler: Takes jobs from DB and pushes them in RabbitMQ
- Agent: Consumes jobs from queue, executes them and pushes results back to queue
- Evaluator: Processes results from queue and pushes notify job if required, writes data in DB; processes from Dead
  Letter Exchange
- Notifier: Sends notification to user

## Agent

### General

- `HEAD` request, more methods as part of #17

### Error Handling

> [!NOTE]
> Exit with error may be replaced with message to Notifier queue for future releases.

- RabbitMQ not reachable: 5 retries -> exit with error
- RabbitMQPublisher, JobService, RabbitMQAdapter cannot be initialized: exit with error
- Error getting messages: exit with error
- Error decoding job: reject with requeue
- Error executing job: reject with requeue ?????
- Unknown job type: log error, reject without requeue -> directly in DLX

#### HTTP

- Request unsuccessful:
  - Check for timeout
  - Cross-check with public website (is the agent network or the target down?) -> subject to discussion
  - If public website unreachable: reject with requeue, exit
  - If public website reachable: target down, reachable false

#### Pinger

- Error with creating pinger: reject with requeue, exit
- Pinging unsuccessful:
  - Check for timeout
  - Cross-check with public website (is the agent network or the target down?) -> subject to discussion
  - If public website unreachable: reject with requeue, exit
  - If public website reachable: target down, reachable false

#### RabbitMQ Publisher

- Error marshaling protobuf: log error, reject with requeue
- Error publishing: log error, reject with requeue, exit

## Job Error Handling

- Dead Letter Exchange after $x$ requeues of message
