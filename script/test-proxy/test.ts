import * as fs from 'fs/promises';

import fetch from 'node-fetch';

async function main() {
  const msg = await fs.readFile('system_prompt.txt', 'utf-8');
  const inputRaw = await fs.readFile('input.json', 'utf-8');
  const data = JSON.parse(inputRaw);

  if (Array.isArray(data.messages) && data.messages.length > 0) {
    data.messages[0].content = msg;
  }

  const start = Date.now();

  const response = await fetch('http://0.0.0.0:4000/chat/completions', {
    method: 'POST',
    headers: {
      Authorization: 'Bearer sk-1234',
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });

  const json = await response.json();
  console.log(JSON.stringify(json, null, 2));
  console.log(json.choices[0].message.content);
  console.log(`Time: ${Date.now() - start} ms`);
}

main().catch(console.error);
