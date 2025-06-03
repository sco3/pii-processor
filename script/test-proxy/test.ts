#!/usr/bin/env -S bash -c "tsc test.ts; node test.js"

import * as fs from "fs/promises";

import fetch from "node-fetch";

async function main() {
  const prompt = await fs.readFile("system_prompt.txt", "utf-8");
  const message = await fs.readFile("example_new_fields.log", "utf-8");
  const inputRaw = await fs.readFile("input.json", "utf-8");
  const data = JSON.parse(inputRaw);

  data.messages[0].content = prompt;
  data.messages[1].content = message;

  const start = Date.now();
  const response = await fetch("http://0.0.0.0:4000/chat/completions", {
    method: "POST",
    headers: {
      Authorization: "Bearer sk-1234",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  const took = Date.now() - start;
  const llm_took = parseFloat(
    response.headers.get("x-litellm-response-duration-ms"),
  );

  const json = await response.json();
  console.log(JSON.stringify(json, null, 2));
  console.log(json.choices[0].message.content);
  console.log(`Time: ${took} ms Extra: ${took - llm_took}`);
}

main().catch(console.error);
