use ductaper::probe::http_probe::HealthProbe;
use ductaper::probe::toggle::Toggle;
use ductaper::util::logging::init_tracing;

use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[tokio::test]
pub async fn test_probe() {
    init_tracing();

    let toggle = Toggle::new("test");
    let mut probe = HealthProbe::new(vec![toggle.clone()], 0);
    let port = probe.start().await;
    info!("Bound port: {}", port);
    sleep(Duration::from_millis(42)).await;
    let client = reqwest::Client::new();
    {
        let url = format!("http://localhost:{}/livez", port);
        if let Ok(r) = client.get(&url).send().await {
            assert_eq!(r.status(), reqwest::StatusCode::OK);
            if let Ok(b) = r.bytes().await {
                assert_eq!(b"Ok", b.as_ref());
            }
        }
    }
    {
        let url = format!("http://localhost:{}/readyz", port);
        if let Ok(r) = client.get(&url).send().await {
            assert_eq!(r.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
            if let Ok(b) = r.bytes().await {
                info!("response: {:?}", String::from_utf8_lossy(&b));
                assert_eq!(b"Waiting for [\"test\"]", b.as_ref());
            }
        }
    }
    toggle.set_ready(true);
    {
        let url = format!("http://localhost:{}/readyz", port);
        if let Ok(r) = client.get(&url).send().await {
            assert_eq!(r.status(), reqwest::StatusCode::OK);
            if let Ok(b) = r.bytes().await {
                info!("response: {:?}", String::from_utf8_lossy(&b));
                assert_eq!(b"Ok", b.as_ref());
            }
        }
    }
    probe.stop().await;
}
