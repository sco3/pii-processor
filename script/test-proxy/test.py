#!/usr/bin/env -S uv run


#!/usr/bin/env python3

import requests, json
from pathlib import Path
from time import time

data = json.load(open("input.json"))

prompt = Path("system_prompt.txt").read_text()
msg= Path("example_new_fields.json").read_text()

data["messages"][0]["content"] = prompt
data["messages"][1]["content"] = msg

with open("/tmp/data.json", "w") as f:
    json.dump(data, f)

start = time()
r = requests.post(
    "http://0.0.0.0:4000/chat/completions",
    headers={"Authorization": "Bearer sk-1234", "Content-Type": "application/json"},
    data=json.dumps(data)
)
took=(time()-start)*1000

print(r.text)
print(json.loads(r.text)["choices"][0]["message"]["content"])
llm_took = float(r.headers.get("x-litellm-response-duration-ms"))
print("LLM Time:",llm_took)

print("Time:",took,"ms", "extra", took-llm_took, "ms")


