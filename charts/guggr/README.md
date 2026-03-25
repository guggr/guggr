

## Parameters

### Global RabbitMQ parameters

These parameters are used by the `agent`, `evaluator` and `scheduler`

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

These parameters are used by the `apiService`, `evaluator` and `scheduler`

| Name                               | Description                                                                          | Value               |
| ---------------------------------- | ------------------------------------------------------------------------------------ | ------------------- |
| `global.postgres.host`             | Global Postgres Host, set to this charts' `postgres.fullname` by default             | `""`                |
| `global.postgres.port`             | Global Postgres Port                                                                 | `5432`              |
| `global.postgres.auth.secretName`  | Name of existing Secret containing the Postgres Username, Password and Database Name | `guggr-postgres`    |
| `global.postgres.auth.usernameKey` | Key in existing Secret containing the Postgres Username                              | `username`          |
| `global.postgres.auth.passwordKey` | Key in existing Secret containing the Postgres Users' Password                       | `postgres-password` |
| `global.postgres.auth.databaseKey` | Key in existing Secret containing the Postgres Database                              | `database`          |

### Api-service Parameters

| Name                          | Description                                                                              | Value  |
| ----------------------------- | ---------------------------------------------------------------------------------------- | ------ |
| `apiService.enabled`          | Enable the Api-service                                                                   | `true` |
| `apiService.replicaCount`     | Number of Api-service replicas to deploy                                                 | `1`    |
| `apiService.logLevel`         | Handed down to `RUST_LOG`. Possible values are `trace`, `debug`, `info`, `warn`, `error` | `info` |
| `apiService.fullnameOverride` | String to fully override `api-service.fullname`                                          | `""`   |
| `apiService.nameOverride`     | String to partially override `api-service.fullname`                                      | `""`   |

### Api-service Image Configuration

| Name                          | Description           | Value                             |
| ----------------------------- | --------------------- | --------------------------------- |
| `apiService.image.repository` | The image repository  | `ghcr.io/guggr/guggr/api-service` |
| `apiService.image.pullPolicy` | The image pull policy | `Always`                          |
| `apiService.image.tag`        | The image tag         | `AppVersion`                      |
| `apiService.imagePullSecrets` | Image pull secrets    | `[]`                              |

### Api-service Service Account

| Name                                    | Description                                                                                                              | Value   |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------- |
| `apiService.serviceAccount.create`      | Specifies whether a service account should be created                                                                    | `false` |
| `apiService.serviceAccount.annotations` | Annotations to add to the service account                                                                                | `{}`    |
| `apiService.serviceAccount.name`        | The name of the service account to use. If not set and create is true, a name is generated using the `fullname` template | `""`    |
| `apiService.serviceAccount.automount`   | Whether to automount the SA token inside the pod                                                                         | `false` |

### Api-service Ingress

| Name                             | Description                        | Value   |
| -------------------------------- | ---------------------------------- | ------- |
| `apiService.ingress.enabled`     | Enable ingress for the Api-service | `false` |
| `apiService.ingress.className`   | Ingress class name                 | `""`    |
| `apiService.ingress.annotations` | Ingress annotations                | `{}`    |
| `apiService.ingress.hosts`       | Ingress hosts configuration        | `{}`    |
| `apiService.ingress.tls`         | Ingress TLS configuration          | `[]`    |

### Api-service Service configuration

| Name                            | Description                | Value  |
| ------------------------------- | -------------------------- | ------ |
| `apiService.service.port`       | Api-service service port   | `80`   |
| `apiService.service.targetPort` | Api-service container port | `8081` |

### Api-service HTTPRoute

| Name                               | Description                                                 | Value   |
| ---------------------------------- | ----------------------------------------------------------- | ------- |
| `apiService.httpRoute.enabled`     | Enable Gateway API HTTPRoute generation for the Api-service | `false` |
| `apiService.httpRoute.annotations` | Additional annotations for the HTTPRoute resource           | `{}`    |
| `apiService.httpRoute.parentRefs`  | References to the parent Gateways                           | `[]`    |
| `apiService.httpRoute.hostnames`   | List of hostnames to match                                  | `[]`    |
| `apiService.httpRoute.rules`       | HTTPRoute rules                                             | `[]`    |

### Api-service Additional Parameters

| Name                            | Description                                           | Value |
| ------------------------------- | ----------------------------------------------------- | ----- |
| `apiService.podAnnotations`     | Map of annotations to add to the pods                 | `{}`  |
| `apiService.podLabels`          | Map of labels to add to the pods                      | `{}`  |
| `apiService.podSecurityContext` | Security Context of the pods                          | `{}`  |
| `apiService.securityContext`    | Container-level security context configuration        | `{}`  |
| `apiService.autoscaling`        | HorizontalPodAutoscaler configuration                 | `{}`  |
| `apiService.nodeSelector`       | Node selector for scheduling pods onto specific nodes | `{}`  |
| `apiService.resources`          | Resource requests and limits for the container        | `{}`  |
| `apiService.tolerations`        | Tolerations for scheduling pods onto tainted nodes    | `[]`  |
| `apiService.affinity`           | Pod affinity and anti-affinity rules                  | `{}`  |

### Api-service Environment variables

| Name                              | Description                                                                      | Value     |
| --------------------------------- | -------------------------------------------------------------------------------- | --------- |
| `apiService.env.port`             | Port the Api-service listens on. Must match with `apiService.service.targetPort` | `8081`    |
| `apiService.env.host`             | Host the Api-service binds to                                                    | `0.0.0.0` |
| `apiService.env.auth_ttl`         | TTL of auth tokens                                                               | `900`     |
| `apiService.env.auth_refresh_ttl` | TTL of refresh tokens                                                            | `2419200` |

### Frontend Parameters

| Name                        | Description                                      | Value  |
| --------------------------- | ------------------------------------------------ | ------ |
| `frontend.enabled`          | Enable the Frontend                              | `true` |
| `frontend.replicaCount`     | Number of Frontend replicas to deploy            | `1`    |
| `frontend.fullnameOverride` | String to fully override `frontend.fullname`     | `""`   |
| `frontend.nameOverride`     | String to partially override `frontend.fullname` | `""`   |

### Frontend Image Configuration

| Name                        | Description           | Value                          |
| --------------------------- | --------------------- | ------------------------------ |
| `frontend.image.repository` | The image repository  | `ghcr.io/guggr/guggr/frontend` |
| `frontend.image.pullPolicy` | The image pull policy | `Always`                       |
| `frontend.image.tag`        | The image tag         | `AppVersion`                   |
| `frontend.imagePullSecrets` | Image pull secrets    | `[]`                           |

### Frontend Service Account

| Name                                  | Description                                                                                                              | Value   |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------- |
| `frontend.serviceAccount.create`      | Specifies whether a service account should be created                                                                    | `false` |
| `frontend.serviceAccount.annotations` | Annotations to add to the service account                                                                                | `{}`    |
| `frontend.serviceAccount.name`        | The name of the service account to use. If not set and create is true, a name is generated using the `fullname` template | `""`    |
| `frontend.serviceAccount.automount`   | Whether to automount the SA token inside the pod                                                                         | `false` |

### Frontend Ingress

| Name                           | Description                     | Value   |
| ------------------------------ | ------------------------------- | ------- |
| `frontend.ingress.enabled`     | Enable ingress for the Frontend | `false` |
| `frontend.ingress.className`   | Ingress class name              | `""`    |
| `frontend.ingress.annotations` | Ingress annotations             | `{}`    |
| `frontend.ingress.hosts`       | Ingress hosts configuration     | `{}`    |
| `frontend.ingress.tls`         | Ingress TLS configuration       | `[]`    |

### Frontend Service configuration

| Name                          | Description                | Value  |
| ----------------------------- | -------------------------- | ------ |
| `frontend.service.port`       | Api-service service port   | `80`   |
| `frontend.service.targetPort` | Api-service container port | `8080` |

### Frontend HTTPRoute

| Name                             | Description                                              | Value   |
| -------------------------------- | -------------------------------------------------------- | ------- |
| `frontend.httpRoute.enabled`     | Enable Gateway API HTTPRoute generation for the Frontend | `false` |
| `frontend.httpRoute.annotations` | Additional annotations for the HTTPRoute resource        | `{}`    |
| `frontend.httpRoute.parentRefs`  | References to the parent Gateways                        | `[]`    |
| `frontend.httpRoute.hostnames`   | List of hostnames to match                               | `[]`    |
| `frontend.httpRoute.rules`       | HTTPRoute rules                                          | `[]`    |

### Frontend Additional Parameters

| Name                          | Description                                           | Value |
| ----------------------------- | ----------------------------------------------------- | ----- |
| `frontend.podAnnotations`     | Map of annotations to add to the pods                 | `{}`  |
| `frontend.podLabels`          | Map of labels to add to the pods                      | `{}`  |
| `frontend.podSecurityContext` | Security Context of the pods                          | `{}`  |
| `frontend.securityContext`    | Container-level security context configuration        | `{}`  |
| `frontend.autoscaling`        | HorizontalPodAutoscaler configuration                 | `{}`  |
| `frontend.nodeSelector`       | Node selector for scheduling pods onto specific nodes | `{}`  |
| `frontend.resources`          | Resource requests and limits for the container        | `{}`  |
| `frontend.tolerations`        | Tolerations for scheduling pods onto tainted nodes    | `[]`  |
| `frontend.affinity`           | Pod affinity and anti-affinity rules                  | `{}`  |

### Agent Parameters

| Name                     | Description                                                                              | Value  |
| ------------------------ | ---------------------------------------------------------------------------------------- | ------ |
| `agent.enabled`          | Enable the Agent                                                                         | `true` |
| `agent.replicaCount`     | Number of Agent replicas to deploy                                                       | `1`    |
| `agent.logLevel`         | Handed down to `RUST_LOG`. Possible values are `trace`, `debug`, `info`, `warn`, `error` | `info` |
| `agent.fullnameOverride` | String to fully override `agent.fullname`                                                | `""`   |
| `agent.nameOverride`     | String to partially override `agent.fullname`                                            | `""`   |

### Agent Image Configuration

| Name                     | Description           | Value                       |
| ------------------------ | --------------------- | --------------------------- |
| `agent.image.repository` | The image repository  | `ghcr.io/guggr/guggr/agent` |
| `agent.image.pullPolicy` | The image pull policy | `Always`                    |
| `agent.image.tag`        | The image tag         | `AppVersion`                |
| `agent.imagePullSecrets` | Image pull secrets    | `[]`                        |

### Agent Service Account

| Name                               | Description                                                                                                              | Value   |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------- |
| `agent.serviceAccount.create`      | Specifies whether a service account should be created                                                                    | `false` |
| `agent.serviceAccount.annotations` | Annotations to add to the service account                                                                                | `{}`    |
| `agent.serviceAccount.name`        | The name of the service account to use. If not set and create is true, a name is generated using the `fullname` template | `""`    |
| `agent.serviceAccount.automount`   | Whether to automount the SA token inside the pod                                                                         | `false` |

### Agent Additional Parameters

| Name                       | Description                                           | Value |
| -------------------------- | ----------------------------------------------------- | ----- |
| `agent.podAnnotations`     | Map of annotations to add to the pods                 | `{}`  |
| `agent.podLabels`          | Map of labels to add to the pods                      | `{}`  |
| `agent.podSecurityContext` | Security Context of the pods                          | `{}`  |
| `agent.securityContext`    | Container-level security context configuration        | `{}`  |
| `agent.autoscaling`        | HorizontalPodAutoscaler configuration                 | `{}`  |
| `agent.nodeSelector`       | Node selector for scheduling pods onto specific nodes | `{}`  |
| `agent.resources`          | Resource requests and limits for the container        | `{}`  |
| `agent.tolerations`        | Tolerations for scheduling pods onto tainted nodes    | `[]`  |
| `agent.affinity`           | Pod affinity and anti-affinity rules                  | `{}`  |

### Evaluator Parameters

| Name                         | Description                                                                              | Value  |
| ---------------------------- | ---------------------------------------------------------------------------------------- | ------ |
| `evaluator.enabled`          | Enable the Evaluator                                                                     | `true` |
| `evaluator.replicaCount`     | Number of Evaluator replicas to deploy                                                   | `1`    |
| `evaluator.logLevel`         | Handed down to `RUST_LOG`. Possible values are `trace`, `debug`, `info`, `warn`, `error` | `info` |
| `evaluator.fullnameOverride` | String to fully override `evaluator.fullname`                                            | `""`   |
| `evaluator.nameOverride`     | String to partially override `evaluator.fullname`                                        | `""`   |

### Evaluator Image Configuration

| Name                         | Description           | Value                           |
| ---------------------------- | --------------------- | ------------------------------- |
| `evaluator.image.repository` | The image repository  | `ghcr.io/guggr/guggr/evaluator` |
| `evaluator.image.pullPolicy` | The image pull policy | `Always`                        |
| `evaluator.image.tag`        | The image tag         | `AppVersion`                    |
| `evaluator.imagePullSecrets` | Image pull secrets    | `[]`                            |

### Evaluator Service Account

| Name                                   | Description                                                                                                              | Value   |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------- |
| `evaluator.serviceAccount.create`      | Specifies whether a service account should be created                                                                    | `false` |
| `evaluator.serviceAccount.annotations` | Annotations to add to the service account                                                                                | `{}`    |
| `evaluator.serviceAccount.name`        | The name of the service account to use. If not set and create is true, a name is generated using the `fullname` template | `""`    |
| `evaluator.serviceAccount.automount`   | Whether to automount the SA token inside the pod                                                                         | `false` |

### Evaluator Additional Parameters

| Name                           | Description                                           | Value |
| ------------------------------ | ----------------------------------------------------- | ----- |
| `evaluator.podAnnotations`     | Map of annotations to add to the pods                 | `{}`  |
| `evaluator.podLabels`          | Map of labels to add to the pods                      | `{}`  |
| `evaluator.podSecurityContext` | Security Context of the pods                          | `{}`  |
| `evaluator.securityContext`    | Container-level security context configuration        | `{}`  |
| `evaluator.autoscaling`        | HorizontalPodAutoscaler configuration                 | `{}`  |
| `evaluator.nodeSelector`       | Node selector for scheduling pods onto specific nodes | `{}`  |
| `evaluator.resources`          | Resource requests and limits for the container        | `{}`  |
| `evaluator.tolerations`        | Tolerations for scheduling pods onto tainted nodes    | `[]`  |
| `evaluator.affinity`           | Pod affinity and anti-affinity rules                  | `{}`  |

### Scheduler Parameters

| Name                         | Description                                                                              | Value  |
| ---------------------------- | ---------------------------------------------------------------------------------------- | ------ |
| `scheduler.enabled`          | Enable the Scheduler                                                                     | `true` |
| `scheduler.replicaCount`     | Number of Scheduler replicas to deploy                                                   | `1`    |
| `scheduler.logLevel`         | Handed down to `RUST_LOG`. Possible values are `trace`, `debug`, `info`, `warn`, `error` | `info` |
| `scheduler.fullnameOverride` | String to fully override `scheduler.fullname`                                            | `""`   |
| `scheduler.nameOverride`     | String to partially override `scheduler.fullname`                                        | `""`   |

### Scheduler Image Configuration

| Name                         | Description           | Value                           |
| ---------------------------- | --------------------- | ------------------------------- |
| `scheduler.image.repository` | The image repository  | `ghcr.io/guggr/guggr/scheduler` |
| `scheduler.image.pullPolicy` | The image pull policy | `Always`                        |
| `scheduler.image.tag`        | The image tag         | `AppVersion`                    |
| `scheduler.imagePullSecrets` | Image pull secrets    | `[]`                            |

### Scheduler Service Account

| Name                                   | Description                                                                                                              | Value   |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------- |
| `scheduler.serviceAccount.create`      | Specifies whether a service account should be created                                                                    | `false` |
| `scheduler.serviceAccount.annotations` | Annotations to add to the service account                                                                                | `{}`    |
| `scheduler.serviceAccount.name`        | The name of the service account to use. If not set and create is true, a name is generated using the `fullname` template | `""`    |
| `scheduler.serviceAccount.automount`   | Whether to automount the SA token inside the pod                                                                         | `false` |

### Scheduler Additional Parameters

| Name                           | Description                                           | Value |
| ------------------------------ | ----------------------------------------------------- | ----- |
| `scheduler.podAnnotations`     | Map of annotations to add to the pods                 | `{}`  |
| `scheduler.podLabels`          | Map of labels to add to the pods                      | `{}`  |
| `scheduler.podSecurityContext` | Security Context of the pods                          | `{}`  |
| `scheduler.securityContext`    | Container-level security context configuration        | `{}`  |
| `scheduler.autoscaling`        | HorizontalPodAutoscaler configuration                 | `{}`  |
| `scheduler.nodeSelector`       | Node selector for scheduling pods onto specific nodes | `{}`  |
| `scheduler.resources`          | Resource requests and limits for the container        | `{}`  |
| `scheduler.tolerations`        | Tolerations for scheduling pods onto tainted nodes    | `[]`  |
| `scheduler.affinity`           | Pod affinity and anti-affinity rules                  | `{}`  |

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


## Example Configurations


### HA-Setup with bundled Postgres & RabbitMQ
```yaml
# ha.yaml
apiService:
  replicaCount: 3

frontend:
  replicaCount: 3

agent:
  replicaCount: 3

evaluator:
  replicaCount: 3

scheduler:
  replicaCount: 3

rabbitmq:
  replicaCount: 3
  peerDiscoveryK8sPlugin:
    enabled: true

postgres:
  replicaCount: 3

```
