import os
import net.http
import x.json2
import time

fn main() {
    start := time.now()
	msg := os.read_file('system_prompt.txt')!

	input_text := os.read_file('input.json') or { panic(err) }
	mut input := json2.raw_decode(input_text) or { panic(err) }

	mut arr := input.as_map()['messages']!.arr()
	mut first := arr[0].as_map()
	first['content'] = json2.Any(msg)
	arr[0] = json2.Any(first)

	input.as_map()['messages'] = json2.Any(arr)

	input_str := input.json_str()
	println(input_str)

	resp := http.post_json('http://0.0.0.0:4000/chat/completions', input_str) or { panic(err) }

	/*
            {
            'Authorization': 'Bearer sk-1234'
            'Content-Type': 'application/json'
        }
	*/

	println(resp.body)

	parsed := json2.raw_decode(resp.body) or { json2.Any{} }
	choices := parsed.as_map()['choices'] or { json2.Any{} }

	if choices is []json2.Any && choices.len > 0 {
		message := choices[0].as_map()['message'] or { json2.Any{} }
		print(message)
		if message is map[string]json2.Any {
			content := message['content'] or { '' }
			println(content)
		}
	}

	error := parsed.as_map()['error'] or { json2.Any{} }
	if error is map[string]json2.Any {
		message := error['message'] or { json2.Any{} }
		if message is string {
			println('str: ${message}}')
		}
	}
    duration := time.since(start).milliseconds()
    println('Time: ${duration} ms')

}
