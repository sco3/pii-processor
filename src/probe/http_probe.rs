use crate::probe::toggle::Toggle;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tracing::info;

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

    pub fn start(&self) -> tokio::task::JoinHandle<()> {
        let router = Router::new()
            .route("/readyz", get(Self::ready_handler))
            .route("/livez", get(Self::live_handler))
            .with_state(self.clone());
        let port = self.port;
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(
                format!("0.0.0.0:{}", port), //
            )
            .await
            .unwrap();

            info!("Probe server running on port {}", port);

            axum::serve(listener, router).await.unwrap();
        })
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
