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
        "content": "I am Joulie Yen. I forgot my <user_input>pin</user_input>",
        "turn": 1
      },
      {
        "role": "assistant",
        "content": "I will help: <assistant_response>asdf</assistant_response>.",
        "turn": 1
      },
    ]
    );
    let s = serde_json::to_string(&log_json).unwrap();
    println!("content: {s}");
    let sl = serde_json::from_str::<SessionLog>(&s).unwrap();
    let txt = get_text_from_session_log(&sl);
    println!("text: {txt}");
    assert_eq!(
        txt, //
        "user: pin\nassistant: asdf\n",
    )
}
#[test]
pub fn test_get_plain_text_no_matching_tags() {
    init_tracing();
    let log_json = json!([
      {
        "role": "user",
        "content": "I am Joulie Yen. I forgot my <user_input>pin.",
        "turn": 1
      },
      {
        "role": "assistant",
        "content": "I will help: <assistant_response>asdf.",
        "turn": 1
      },
    ]
    );
    let s = serde_json::to_string(&log_json).unwrap();
    println!("content: {s}");
    let sl = serde_json::from_str::<SessionLog>(&s).unwrap();
    let txt = get_text_from_session_log(&sl);
    println!("text: {txt}");
    assert_eq!(
        txt, //
        "user: I am Joulie Yen. I forgot my pin.\nassistant: I will help: asdf.\n",
    )
}

#[test]
pub fn generate_text_from_file() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read("tests/data/worker-pool-test.json")?;

    let log = serde_json::from_slice::<SessionLog>(&content)?;
    let txt = get_text_from_session_log(&log);
    println!("Len: {}", txt.len());
    Ok(())
}
