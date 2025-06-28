#!/usr/bin/env -S bash

curl -v -s -X POST \
      http://localhost:8787/v1/chat/completions \
      -H 'x-portkey-config: {"cache":{"mode":"simple"}}' \
      -H "Content-Type: application/json" \
      -H "x-portkey-provider: bedrock" \
-H "x-portkey-aws-access-key-id: ASIA..." \
-H "x-portkey-aws-secret-access-key: GN4W..." \
-H "x-portkey-aws-region: us-east-1" \
-H "x-portkey-aws-session-token: IQoJ..." \
      -d '{
        "messages": [
            { "role": "user", "content": "Hello, how are you?" }
        ],
        "model": "anthropic.claude-3-haiku-20240307-v1:0",
        "temperature":0
      }'
