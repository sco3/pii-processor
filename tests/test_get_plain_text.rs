use ductaper::data::session_log_models::SessionLog;
use ductaper::llm_work::get_text_from_session_log::get_text_from_session_log;
use ductaper::util::logging::init_tracing;
use serde_json::json;

#[test]
pub fn test_get_plain_text() {
    init_tracing();
    let log_json = json!([
      {
        "role": "user",
        "content": "I am Joulie Yen? I forgot my <user_input>pin</user_input>",
        "turn": 1
      },
      {
        "role": "assistant",
        "content": "I am Joulie Yen? I forgot my <assistant_response>asdf</assistant_response>",
        "turn": 1
      },
    ]
    );
    let s = serde_json::to_string(&log_json).unwrap();
    println!("content: {}", s);
    let sl = serde_json::from_str::<SessionLog>(&s).unwrap();
    let txt = get_text_from_session_log(&sl);
    println!("text: {}", txt);
    assert_eq!(
        txt, //
        "user: pin\nassistant: asdf\n",
    )
}
