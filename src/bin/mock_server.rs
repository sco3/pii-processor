use axum::routing::post;
use axum::Router;
use ductaper::util::exit_codes::ExitCode;
use std::process::exit;
use axum::body::Body;
use axum::http::{header, Response};

#[tokio::main]
pub async fn main() {
    let content = std::fs::read_to_string("response.json").unwrap();
    let router = Router::new().route(
        "/chat/completions", //
        post(|| async {
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(content))
                .unwrap()            
            
            }),
    );

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:4000").await {
        Ok(l) => l,
        Err(e) => {
            println!("Server failed to start:: {}", e);
            exit(-1);
        }
    };

    if let Err(e) = axum::serve(listener, router).await {
        println!("Probe serve error: {}", e);
        exit(ExitCode::ProbeError.code());
    }
}
