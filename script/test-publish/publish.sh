#!/usr/bin/env -S bash

set -xueo pipefail

nats pub localhost.localhost.redact-log "$(cat  ../../tests/data/to_update.json )" --header=session_log_name:redacted.json