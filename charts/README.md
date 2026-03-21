

## Parameters

### Global RabbitMQ parameters

These parameters are used by the `agent`, `evaluator` and `scheduler` Subcharts

| Name                               | Description                                                                                                                                            | Value               |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------- |
| `global.rabbitmq.host`             | Global RabbitMQ set to this charts' `rabbitmq.fullname` by default                                                                                     | `""`                |
| `global.rabbitmq.port`             | Global RabbitMQ AMQP port                                                                                                                              | `5672`              |
| `global.rabbitmq.jobsQueue`        | queue for jobs. Must match the definition in [rabbitmq_definitions.json](files/rabbitmq_definitions.json) when using the bundled RabbitMQ chart        | `scheduler.jobs`    |
| `global.rabbitmq.jobResultQueue`   | queue for job results. Must match the definition in [rabbitmq_definitions.json](files/rabbitmq_definitions.json) when using the bundled RabbitMQ chart | `agent.job_results` |
| `global.rabbitmq.auth.username`    | Username of the RabbitMQ User                                                                                                                          | `guggr`             |
| `global.rabbitmq.auth.secretName`  | Name of existing Secret containing the RabbitMQ Users' Password                                                                                        | `guggr-rabbitmq`    |
| `global.rabbitmq.auth.passwordKey` | Key in existing Secret containing the RabbitMQ Users' Password                                                                                         | `password`          |

### Global Postgres parameters

These parameters are used by the `api-service`, `evaluator` and `scheduler` Subcharts

| Name                               | Description                                                                          | Value               |
| ---------------------------------- | ------------------------------------------------------------------------------------ | ------------------- |
| `global.postgres.host`             | Global Postgres Host, set to this charts' `postgres.fullname` by default             | `""`                |
| `global.postgres.port`             | Global Postgres Port                                                                 | `5432`              |
| `global.postgres.auth.secretName`  | Name of existing Secret containing the Postgres Username, Password and Database Name | `guggr-postgres`    |
| `global.postgres.auth.usernameKey` | Key in existing Secret containing the Postgres Username                              | `username`          |
| `global.postgres.auth.passwordKey` | Key in existing Secret containing the Postgres Users' Password                       | `postgres-password` |
| `global.postgres.auth.databaseKey` | Key in existing Secret containing the Postgres Database                              | `database`          |

### Api-service Parameters

| Name                           | Description                                            | Value                             |
| ------------------------------ | ------------------------------------------------------ | --------------------------------- |
| `api-service.enabled`          | Enable the Api-service                                 | `true`                            |
| `api-service.replicaCount`     | Number of Api-service replicas to deploy               | `1`                               |
| `api-service.image`            | Optional image override for the Api-service container. | `{}`                              |
| `api-service.image.repository` | Override the image repository.                         | `ghcr.io/guggr/guggr/api-service` |
| `api-service.image.tag`        | Override the image tag.                                | `AppVersion`                      |

### Api-service Ingress

| Name                              | Description                        | Value   |
| --------------------------------- | ---------------------------------- | ------- |
| `api-service.ingress.enabled`     | Enable ingress for the Api-service | `false` |
| `api-service.ingress.className`   | Ingress class name                 | `""`    |
| `api-service.ingress.annotations` | Ingress annotations                | `{}`    |
| `api-service.ingress.hosts`       | Ingress hosts configuration        | `{}`    |
| `api-service.ingress.tls`         | Ingress TLS configuration          | `[]`    |

### Api-service HTTPRoute

| Name                                | Description                                                 | Value   |
| ----------------------------------- | ----------------------------------------------------------- | ------- |
| `api-service.httpRoute.enabled`     | Enable Gateway API HTTPRoute generation for the Api-service | `false` |
| `api-service.httpRoute.annotations` | Additional annotations for the HTTPRoute resource           | `{}`    |
| `api-service.httpRoute.parentRefs`  | References to the parent Gateways                           | `[]`    |
| `api-service.httpRoute.hostnames`   | List of hostnames to match                                  | `[]`    |
| `api-service.httpRoute.rules`       | HTTPRoute rules                                             | `[]`    |

### Frontend Parameters

| Name                        | Description                                         | Value                          |
| --------------------------- | --------------------------------------------------- | ------------------------------ |
| `frontend.enabled`          | Enable the Frontend                                 | `true`                         |
| `frontend.replicaCount`     | Number of Frontend replicas to deploy               | `1`                            |
| `frontend.image`            | Optional image override for the Frontend container. | `{}`                           |
| `frontend.image.repository` | Override the image repository.                      | `ghcr.io/guggr/guggr/frontend` |
| `frontend.image.tag`        | Override the image tag.                             | `AppVersion`                   |

### Frontend Ingress

| Name                           | Description                     | Value   |
| ------------------------------ | ------------------------------- | ------- |
| `frontend.ingress.enabled`     | Enable ingress for the Frontend | `false` |
| `frontend.ingress.className`   | Ingress class name              | `""`    |
| `frontend.ingress.annotations` | Ingress annotations             | `{}`    |
| `frontend.ingress.hosts`       | Ingress hosts configuration     | `{}`    |
| `frontend.ingress.tls`         | Ingress TLS configuration       | `[]`    |

### Frontend HTTPRoute

| Name                             | Description                                              | Value   |
| -------------------------------- | -------------------------------------------------------- | ------- |
| `frontend.httpRoute.enabled`     | Enable Gateway API HTTPRoute generation for the Frontend | `false` |
| `frontend.httpRoute.annotations` | Additional annotations for the HTTPRoute resource        | `{}`    |
| `frontend.httpRoute.parentRefs`  | References to the parent Gateways                        | `[]`    |
| `frontend.httpRoute.hostnames`   | List of hostnames to match                               | `[]`    |
| `frontend.httpRoute.rules`       | HTTPRoute rules                                          | `[]`    |

### Agent Parameters

| Name                     | Description                                      | Value                       |
| ------------------------ | ------------------------------------------------ | --------------------------- |
| `agent.enabled`          | Enable the Agent                                 | `true`                      |
| `agent.replicaCount`     | Number of Agent replicas to deploy               | `1`                         |
| `agent.image`            | Optional image override for the Agent container. | `{}`                        |
| `agent.image.repository` | Override the image repository.                   | `ghcr.io/guggr/guggr/agent` |
| `agent.image.tag`        | Override the image tag.                          | `AppVersion`                |

### Evaluator Parameters

| Name                         | Description                                          | Value                           |
| ---------------------------- | ---------------------------------------------------- | ------------------------------- |
| `evaluator.enabled`          | Enable the Evaluator                                 | `true`                          |
| `evaluator.replicaCount`     | Number of Evaluator replicas to deploy               | `1`                             |
| `evaluator.image`            | Optional image override for the Evaluator container. | `{}`                            |
| `evaluator.image.repository` | Override the image repository.                       | `ghcr.io/guggr/guggr/evaluator` |
| `evaluator.image.tag`        | Override the image tag.                              | `AppVersion`                    |

### Scheduler Parameters

| Name                         | Description                                          | Value                           |
| ---------------------------- | ---------------------------------------------------- | ------------------------------- |
| `scheduler.enabled`          | Enable the Scheduler                                 | `true`                          |
| `scheduler.replicaCount`     | Number of Scheduler replicas to deploy               | `1`                             |
| `scheduler.image`            | Optional image override for the Scheduler container. | `{}`                            |
| `scheduler.image.repository` | Override the image repository.                       | `ghcr.io/guggr/guggr/scheduler` |
| `scheduler.image.tag`        | Override the image tag.                              | `AppVersion`                    |

### Postgres Parameters

| Name                     | Description                                                                                                    | Value   |
| ------------------------ | -------------------------------------------------------------------------------------------------------------- | ------- |
| `postgres.enabled`       | Enable the bundled Postgres Subchart                                                                           | `true`  |
| `postgres.replicaCount`  | Number of PostgreSQL replicas to deploy (Note: PostgreSQL doesn't support multi-master replication by default) | `1`     |
| `postgres.auth.username` | Name for default User that is created at startup                                                               | `guggr` |
| `postgres.auth.database` | Name for default Database that is created at startup                                                           | `guggr` |

### RabbitMQ Parameters

| Name                                    | Description                                                                                        | Value   |
| --------------------------------------- | -------------------------------------------------------------------------------------------------- | ------- |
| `rabbitmq.enabled`                      | Enable the bundled RabbitMQ Subchart                                                               | `true`  |
| `rabbitmq.replicaCount`                 | Number of RabbitMQ replicas to deploy (clustering needs to be enabled to set more than 1 replicas) | `1`     |
| `rabbitmq.auth.enabled`                 | Enable RabbitMQ Authentication                                                                     | `true`  |
| `rabbitmq.auth.username`                | RabbitMQ default username                                                                          | `guggr` |
| `rabbitmq.managementPlugin.enabled`     | Enable RabbitMQ management plugin                                                                  | `false` |
| `rabbitmq.extraVolumes`                 | Extra volumes to add to RabbitMQ pods                                                              | `[]`    |
| `rabbitmq.extraVolumeMounts`            | Extra volumes to add to RabbitMQ pods                                                              | `[]`    |
| `rabbitmq.customScripts.postStart`      |                                                                                                    | `{}`    |
| `rabbitmq.customScripts.initContainers` | Custom init containers to run before RabbitMQ starts                                               | `[]`    |
