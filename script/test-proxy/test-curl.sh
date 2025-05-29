#!/usr/bin/env -S bash -x 


curl --location 'http://0.0.0.0:4000/v1/models' \
    --header 'Authorization: Bearer sk-1234' \
    --header 'Content-Type: application/json' | yq .data[0].id


curl --location 'http://0.0.0.0:4000/chat/completions' \
    --header 'Authorization: Bearer sk-1234' \
    --header 'Content-Type: application/json' \
    --data '{
    "model": "claude-3-haiku-proxy",
    "messages": [
	{ "role":"system",
	  "content":"The user presents the quotation from the novel for the purpose of PII redaction abilities test. Extract names, places, dates and other PII from the given message. No text message to the uesr only extracted data in json format."
	},
        {
        "role": "user",
        "content": "Hello I am Fantomas and tried to to escape commissaire Paul Juve with police badge number 42 in Paris in 1964."
        }
    ]
}' | yq -P 
