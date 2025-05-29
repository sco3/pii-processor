use crate::probe::http_probe::HealthProbe;
use crate::probe::toggle::Toggle;
use crate::util::exit_codes::ExitCode;
use axum::Router;
use axum::routing::get;
use std::process::exit;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info};

#[derive(Clone)]
/// http server state
pub struct ProbeState {
    /// components to check readiness
    pub(crate) components: Arc<Vec<Toggle>>,
    // /// actual listen port number
    // pub(crate) port: u16,
    /// entry points list
    pub(crate) entrypoints: String,
}

impl HealthProbe {
    /// stops probe
    pub async fn stop(&mut self) {
        if let Some(stop) = self.stop_tx.take() {
            let _ = stop.send(());
        }
        if let Some(handle) = self.join_handle.take() {
            let _ = handle.await;
        }
    }
    /// starts probe method
    pub async fn start(&mut self) -> u16 {
        let (stop_tx, stop_rx) = oneshot::channel::<()>();
        self.stop_tx = Some(stop_tx);

        let entrypoints =
            serde_json::to_string(&crate::probe::http_probe::ENTRYPOINTS).unwrap_or_default();

        let state = ProbeState {
            components: Arc::clone(&self.components),
            // port: self.port,
            entrypoints,
        };

        let router = Router::new()
            .route(crate::probe::http_probe::ROOT, get(Self::root_handler))
            .route(crate::probe::http_probe::READYZ, get(Self::ready_handler))
            .route(crate::probe::http_probe::LIVEZ, get(Self::live_handler))
            .with_state(state);

        let port = self.port;

        let addr = format!("127.0.0.1:{port}");
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                error!("Probe failed to start:: {}", e);
                exit(ExitCode::ProbeError.code());
            }
        };

        let port = match listener.local_addr() {
            Ok(addr) => addr.port(),
            Err(_) => 0,
        };
        let join_handle = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, router) //
                .with_graceful_shutdown(async {
                    stop_rx.await.ok();
                    info!("Stop probe");
                })
                .await
            {
                error!("Probe serve error: {}", e);
                exit(ExitCode::ProbeError.code());
            }
            info!("Probe stopped");
        });
        self.join_handle = Some(join_handle);

        port
    }
}
