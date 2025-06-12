#!/usr/bin/env -S bash 

set -xueo pipefail

curl --location 'http://0.0.0.0:4000/v1/models' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' | yq -P 
	


export PROMPT=$(<system_prompt.txt )
export MSG=$(<worker-pool-test.json )


yq eval '.messages[0].content = strenv(PROMPT)' input.json -o json > /tmp/curl-prompt.json

yq eval '.messages[1].content = strenv(MSG)' /tmp/curl-prompt.json -o json > /tmp/curl-data.json

rm -rf /tmp/curl-out*

time curl -s -v 'http://localhost:4000/chat/completions' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' \
	--data @/tmp/curl-data.json > /tmp/curl-out.json
yq -P /tmp/curl-out.json > /tmp/curl-out-formatted.json

yq .choices[0].message.content -r -P -o yaml /tmp/curl-out.json > /tmp/curl-out-content.txt
yq .choices[0].message.content -r -P -o yaml /tmp/curl-out.json  | yq -P

