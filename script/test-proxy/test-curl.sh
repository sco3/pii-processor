#!/usr/bin/env -S bash 

set -xueo pipefail

curl --location 'http://0.0.0.0:4000/v1/models' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' | yq .data[0].id
	



export PROMPT=$(<system_prompt.txt )
export MSG=$(<example_new_fields.log )


yq eval '.messages[0].content = strenv(PROMPT)' input.json -o json > /tmp/data1.json

yq eval '.messages[1].content = strenv(MSG)' /tmp/data1.json -o json > /tmp/data.json

rm -f /tmp/out*.yaml
rm -f  /tmp/out*.json


time curl -s -v 'http://0.0.0.0:4000/chat/completions' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' \
	--data @/tmp/data.json > /tmp/out.json


yq .choices[0].message.content -r -P -o yaml /tmp/out.json > /tmp/out1.yaml
# yq -P -o yaml /tmp/out1.json


