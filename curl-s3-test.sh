#!/usr/bin/env -S bash


port=$(docker ps --format '{{json .}}' | grep s3mock | yq -P | yq .Ports | tr ',' '\n' | grep 0.0.0.0| grep 9090 | awk -F- '{ print $1 }' | awk -F: '{ print $2}')



curl  --request PUT --upload-file ./Cargo.toml http://localhost:$port/test-bucket/asdf3


aws s3api list-objects-v2  --bucket test-bucket --endpoint-url=http://localhost:$port


