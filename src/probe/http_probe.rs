use crate::probe::toggle::Toggle;
use crate::util::exit_codes::ExitCode;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::process::exit;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct HealthProbe {
    components: Arc<Vec<Toggle>>,
    port: u16,
    entrypoints: String,
}
const ROOT: &str = "/";
const READYZ: &str = "/readyz";
const LIVEZ: &str = "/livez";
const ENTRYPOINTS: [&str; 3] = [ROOT, READYZ, LIVEZ];

impl HealthProbe {
    pub fn new(components: Vec<Toggle>, port: u16) -> Self {
        let entrypoints = serde_json::to_string(&ENTRYPOINTS).unwrap_or_default();
        Self {
            components: Arc::new(components),
            port,
            entrypoints,
        }
    }

    pub async fn start(&self) -> u16 {
        let (tx, rx) = oneshot::channel();
        //let bound_port: u16 = 0;
        let router = Router::new()
            .route(ROOT, get(Self::root_handler))
            .route(READYZ, get(Self::ready_handler))
            .route(LIVEZ, get(Self::live_handler))
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
            if let Ok(bound) = listener.local_addr() {
                info!("Probe listening to: {}", bound);
                if let Err(e) = tx.send(bound.port()) {
                    warn!("Cannot get port for tests: {}", e);
                }
            }

            if let Err(e) = axum::serve(listener, router).await {
                error!("Probe serve error: {}", e);
                exit(ExitCode::ProbeError.code());
            }
        });
        if let Ok(port) = rx.await {
            return port;
        }
        0
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
    async fn root_handler(State(probe): State<HealthProbe>) -> impl IntoResponse {
        (StatusCode::OK, probe.entrypoints)
    }
}
