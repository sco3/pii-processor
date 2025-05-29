#!/usr/bin/env -S bash


curl -s -v http://localhost:11434/v1/chat/completions \
  -H "Content-Type: application/json" \
  --data @test-ollama.json
