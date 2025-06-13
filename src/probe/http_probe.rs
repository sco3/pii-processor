use crate::probe::toggle::Toggle;
use crate::util::exit_codes::ExitCode;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use std::process::exit;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Clone)]
pub struct HealthProbe {
    components: Arc<Vec<Toggle>>,
    port: u16,
}

impl HealthProbe {
    pub fn new(components: Vec<Toggle>, port: u16) -> Self {
        Self {
            components: Arc::new(components),
            port,
        }
    }

    pub fn start(&self) {
        let router = Router::new()
            .route("/readyz", get(Self::ready_handler))
            .route("/livez", get(Self::live_handler))
            .with_state(self.clone());
        let port = self.port;
        tokio::spawn(async move {
            let addr = format!("127.0.0.1:{}", port);
            let listener = match tokio::net::TcpListener::bind(&addr).await {
                Ok(l) => l,
                Err(e) => {
                    error!("Probe failed to start:: {}", e);
                    exit(ExitCode::ProbeError.code());
                }
            };
            info!("Probe listening to: {}", addr);
            if let Err(e) = axum::serve(listener, router).await {
                error!("Probe serve error: {}", e);
                exit(ExitCode::ProbeError.code());
            }
        });
    }

    async fn ready_handler(State(state): State<Self>) -> impl IntoResponse {
        let not_ready: Vec<String> = state
            .components //
            .iter()
            .filter(|c| !c.is_ready())
            .map(|t| t.name())
            .collect();

        if not_ready.is_empty() {
            (StatusCode::OK, "Ok".to_string())
        } else {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Waiting for {:?}", not_ready).clone(),
            )
        }
    }

    async fn live_handler() -> impl IntoResponse {
        (StatusCode::OK, "Ok")
    }
}
