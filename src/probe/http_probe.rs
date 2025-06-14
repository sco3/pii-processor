use crate::probe::toggle::Toggle;
use crate::util::exit_codes::ExitCode;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use std::process::exit;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct HealthProbe {
    components: Arc<Vec<Toggle>>,
    port: u16,
    entrypoints: String,
}

pub struct HealthProbeStart {
    pub port: u16,
    pub stop_tx: Sender<()>,
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
    pub async fn stop(&self) {}

    pub async fn start(&self) -> HealthProbeStart {
        let (port_tx, port_rx) = oneshot::channel();
        let (stop_tx, stop_rx) = oneshot::channel::<()>();
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
                if let Err(e) = port_tx.send(bound.port()) {
                    warn!("Cannot get port for tests: {}", e);
                }
            }

            if let Err(e) = axum::serve(listener, router) //
                .with_graceful_shutdown(Self::stop_signal(stop_rx))
                .await
            {
                error!("Probe serve error: {}", e);
                exit(ExitCode::ProbeError.code());
            }
        });
        let port = port_rx.await.unwrap_or_default();

        HealthProbeStart { port, stop_tx }
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
    async fn stop_signal(stop_rx: Receiver<()>) {
        let _ = stop_rx.await;
    }
}
