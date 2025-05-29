use crate::probe::http_probe_start::ProbeState;
/// live/healty http prove
use crate::probe::toggle::Toggle;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;

/// structore for http probe
pub struct HealthProbe {
    /// components to check readiness
    pub(crate) components: Arc<Vec<Toggle>>,
    /// actual listen port number
    pub(crate) port: u16,
    /// sender to stop probe when started sends signal probe exits
    pub stop_tx: Option<Sender<()>>,
    /// join handler for graceful shutdown
    pub join_handle: Option<JoinHandle<()>>,
}

/// root endpoint
pub(crate) const ROOT: &str = "/";

/// ready endpoint
pub(crate) const READYZ: &str = "/readyz";

//live endpoint
pub(crate) const LIVEZ: &str = "/livez";

/// all endpoints
pub(crate) const ENTRYPOINTS: [&str; 3] = [ROOT, READYZ, LIVEZ];

/// methods for http probe
impl HealthProbe {
    ///constructor
    #[must_use]
    pub fn new(components: Vec<Toggle>, port: u16) -> Self {
        Self {
            components: Arc::new(components),
            port,
            stop_tx: None,
            join_handle: None,
        }
    }

    #[allow(clippy::unused_async)] // handlers must be async
    /// response to /readyz calls - it checks all registered components
    /// for ready status, for instance result of s3 operation may affect
    /// readiness respose
    pub(crate) async fn ready_handler(State(state): State<ProbeState>) -> impl IntoResponse {
        let not_ready: Vec<String> = state
            .components //
            .iter()
            .filter(|c| c.not_ready())
            .map(super::toggle::Toggle::name)
            .collect();

        if not_ready.is_empty() {
            (StatusCode::OK, "Ok".to_string())
        } else {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Waiting for {not_ready:?}").clone(),
            )
        }
    }

    #[allow(clippy::unused_async)] // endpoint handlers must be async
    /// liveness probe response
    pub(crate) async fn live_handler() -> impl IntoResponse {
        (StatusCode::OK, "Ok")
    }

    #[allow(clippy::unused_async)] // endpoint handlers must be async
    /// reponse to / endpoint - nats server returns valid endpoint list
    /// lets do the same it is convenient.
    pub(crate) async fn root_handler(State(state): State<ProbeState>) -> impl IntoResponse {
        (StatusCode::OK, state.entrypoints)
    }
}
