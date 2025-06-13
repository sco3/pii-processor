use ductaper::probe::http_probe::HealthProbe;
use ductaper::probe::toggle::Toggle;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
pub async fn test_probe() {
    let toggle = Toggle::new("test");
    let probe = HealthProbe::new(vec![toggle], 0);
    probe.start();
    sleep(Duration::from_millis(42)).await;
}
