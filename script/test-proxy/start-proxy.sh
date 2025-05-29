#!/usr/bin/env -S bash

set -xueo pipefail

env=~/.local/.env-ductaper

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

tmux new -d -s litellm uv run litellm --debug --config config.yaml

clear
while true; do 
   out=$(netstat -ltnp | grep -Eo ':4000\b' || echo Not yet )
   if [ "$out" == ":4000" ]; then
      break
   fi
   sleep 1
done
