#!/usr/bin/env -S bash

set -xueo pipefail

name=$(names)
nats pub localhost.customer_support.redact_log "$(cat to_update.json)" --header="session_log_name:${name}.log"

echo $name.log
