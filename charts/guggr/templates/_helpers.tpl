{{/*
Expand the name of the chart.
*/}}
{{- define "guggr.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "guggr.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "guggr.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "guggr.labels" -}}
helm.sh/chart: {{ include "guggr.chart" . }}
{{ include "guggr.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "guggr.selectorLabels" -}}
app.kubernetes.io/name: {{ include "guggr.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "guggr.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "guggr.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{- /*
Postgres Host is either taken from global.postgres.host or if it's nil, from the Release Name with `-postgres` appended
*/}}
{{- define "guggr.postgresHost" -}}
{{- if .Values.global.postgres.host -}}
{{- .Values.global.postgres.host -}}
{{- else -}}
{{- printf "%s-postgres" .Release.Name -}}
{{- end -}}
{{- end -}}

{{- /*
RabbitMQ Host is either taken from global.rabbitmq.host or if it's nil, from the Release Name with `-rabbitmq` appended
*/}}
{{- define "guggr.rabbitmqHost" -}}
{{- if .Values.global.rabbitmq.host -}}
{{- .Values.global.rabbitmq.host -}}
{{- else -}}
{{- printf "%s-rabbitmq" .Release.Name -}}
{{- end -}}
{{- end -}}

{{- /*
init container template that waits till the Postgres port is open
*/}}
{{- define "guggr.waitForPostgres" -}}
- name: wait-for-postgres
  image: busybox:1.37
  securityContext:
    runAsUser: 65532
    runAsGroup: 65532
    allowPrivilegeEscalation: false
    readOnlyRootFilesystem: true
    capabilities:
      drop:
        - ALL
  command:
    - sh
    - -ec
    - |
      until nc -z {{ include "guggr.postgresHost" . }} {{ .Values.global.postgres.port }}; do
        echo "waiting for postgres"
        sleep 2
      done
{{- end -}}

{{- /*
init container template that waits till the RabbitMQ AMQP port is open
*/}}
{{- define "guggr.waitForRabbitMQ" -}}
- name: wait-for-rabbitmq
  image: busybox:1.37
  securityContext:
    runAsUser: 65532
    runAsGroup: 65532
    allowPrivilegeEscalation: false
    readOnlyRootFilesystem: true
    capabilities:
      drop:
        - ALL
  command:
    - sh
    - -ec
    - |
      until nc -z {{ include "guggr.rabbitmqHost" . }} {{ .Values.global.rabbitmq.port }}; do
        echo "waiting for rabbitmq"
        sleep 2
      done
{{- end -}}


{{/*
Generate the application url
*/}}
{{- define "guggr.url" -}}
{{- if .Values.frontend.ingress.enabled -}}
{{- $host := index .Values.frontend.ingress.hosts 0 -}}
{{- if .Values.frontend.ingress.tls -}}
https://{{ $host.host }}
{{- else -}}
http://{{ $host.host }}
{{- end -}}
{{- end -}}
{{- end -}}
