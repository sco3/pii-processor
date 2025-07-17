use serde_json::json;

pub async fn dummy_llm_caller() {
    let _dummy_response = json!(
    {"choices": [
        {
          "finish_reason": "stop",
          "index": 0,
          "message": {
            "content": "{\n\"redactions\": {\n\"Joulie Yen\":\"[PERSON]\"\n}\n}",
            "function_call": null,
            "role": "assistant",
            "tool_calls": null
          }
        }
      ],
      "created": 1749678492,
      "id": "chatcmpl-b6ec4f61-1656-48d5-b604-31bbb3815378",
      "model": "anthropic.claude-3-haiku-20240307-v1:0",
      "object": "chat.completion",
      "system_fingerprint": null,
      "usage": {
        "cache_creation_input_tokens": 0,
        "cache_read_input_tokens": 0,
        "completion_tokens": 31,
        "completion_tokens_details": null,
        "prompt_tokens": 5850,
        "prompt_tokens_details": {
          "audio_tokens": null,
          "cached_tokens": 0
        },
        "total_tokens": 5881
      }
    });
}
