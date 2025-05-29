#!/usr/bin/env -S bash -x

curl --location 'http://0.0.0.0:4000/v1/models' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' | yq .data[0].id
	
	
	
msg="$(cat system_prompt.txt)"
escaped=$(jq -Rs <<< "$msg")  # Read raw string and escape it

yq ".messages[0].content = $escaped" data.yaml -o json > data.json

time curl -s 'http://0.0.0.0:4000/chat/completions' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' \
	--data @data.json > out.json
cat out.json
yq -P .choices[0].message.content -o yaml out.json | yq -P -o yaml
