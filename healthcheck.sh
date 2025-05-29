#!/usr/bin/env bash

PORT=${REDACT_PROBE_PORT:-8118}
TIMEOUT=0.25
SLEEP=1
RETRIES=20
MAX_TIME=10

if [[ "${1}" == "-v" ]]; then
  set -x
fi

curl -s \
  --max-time $MAX_TIME \
  --retry $RETRIES \
  --retry-delay $SLEEP \
  --connect-timeout $TIMEOUT \
  http://localhost:${PORT}/healthz;
