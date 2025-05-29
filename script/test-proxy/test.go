package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"time"
)

func main() {
	msgBytes, _ := ioutil.ReadFile("system_prompt.txt")

	inputFile, _ := os.Open("input.json")
	defer inputFile.Close()
	var data map[string]interface{}
	json.NewDecoder(inputFile).Decode(&data)

	if messages, ok := data["messages"].([]interface{}); ok {
		if len(messages) > 0 {
			if firstMsg, ok := messages[0].(map[string]interface{}); ok {
				firstMsg["content"] = string(msgBytes)
			}
		}
	}

	payloadBytes, _ := json.Marshal(data)

	start := time.Now()
	req, _ := http.NewRequest("POST", "http://0.0.0.0:4000/chat/completions", bytes.NewReader(payloadBytes))
	req.Header.Set("Authorization", "Bearer sk-1234")
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, _ := client.Do(req)
	defer resp.Body.Close()
	body, _ := ioutil.ReadAll(resp.Body)

	fmt.Println(string(body))

	var respData map[string]interface{}
	json.Unmarshal(body, &respData)
	choices := respData["choices"].([]interface{})
	choice := choices[0].(map[string]interface{})
	message := choice["message"].(map[string]interface{})
	content := message["content"].(string)
	fmt.Println(content)

	fmt.Printf("Time: %d ms\n", time.Since(start).Milliseconds())
}
