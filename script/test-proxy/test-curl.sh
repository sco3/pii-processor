#!/usr/bin/env -S bash -x

curl --location 'http://0.0.0.0:4000/v1/models' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' | yq .data[0].id
	
	
msg=

time curl -s 'http://0.0.0.0:4000/chat/completions' \
	--header 'Authorization: Bearer sk-1234' \
	--header 'Content-Type: application/json' \
	--data '{
    "model": "claude-3-haiku-proxy",
    "messages": [
	{ "role":"system",
	  "content":"The user presents the text for personal identifiable information redaction. Extract names, places, dates, identifiers and other PII from the given message. Yaml result should an array with no tags with colons only plain strings .  No text message to the uesr only extracted data in yaml format."
	},
        {
        "role": "user",
        "content": "Hello I am Fantomas and tried to to escape commissaire Paul Juve with police badge number 42 in Paris in 1964. A phone number +44-2244 was in use lately."
        }
    ]
}' | yq -P .choices[0].message.content
