#!/usr/bin/env -S bash

set -xueo pipefail

nats pub localhost.customer_support.redact_log "$(cat to_update.json)" --header="session_log_name:$(names).log"
