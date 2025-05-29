#!/usr/bin/env -S bash

set -xueo pipefail

env=~/.local/.env-ductaper

docker rm -f litellm

if [ ! -f $env ]; then
	echo $env with AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION not found
	echo Use ~/.aws/credentials
	export AWS_ACCESS_KEY_ID=$(echo -n $(cat ~/.aws/credentials | grep aws_access_key_id | awk -F= '{ print $2}'))
	export AWS_SECRET_ACCESS_KEY=$(echo $(cat ~/.aws/credentials | grep aws_secret_access_key | awk -F= '{ print $2}'))
	export AWS_REGION=us-east-1
else
	echo use $env
	source $env
fi

#tmux new -d -s litellm uv run litellm --debug --config config.yaml

docker run -d  \
    -v $(pwd)/config.yaml:/app/config.yaml \
    -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
    -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY \
    -e AWS_REGION=$AWS_REGION \
    -p 4000:4000 \
    --name litellm \
    ghcr.io/berriai/litellm:main-latest \
    --config /app/config.yaml --detailed_debug

clear
while true; do
   out=$(netstat -ltnp | grep -Eo ":4000\b" | head -n 1 | tr -d ' ' || echo Not yet )
   if [ "$out" == ":4000"  ]; then
      break
   fi
   sleep 1
done
