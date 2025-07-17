mod common;

use crate::common::init_cfg::get_test_cfg;
use ductaper::probe::toggle::Toggle;
use ductaper::storage::local_saver::LocalSaver;
use ductaper::storage::saver::Saver;
use ductaper::storage::saver_factory::get_saver;
use ductaper::util::logging::init_tracing;

#[tokio::test]
pub async fn test_local_saver() {
    init_tracing();
    let session_log = Vec::new();
    let saver = LocalSaver::new("/tmp/tmp");
    let r = saver.save(session_log, "asdf.json").await;
    assert!(r);
    let name = saver.get_name();
    assert!(!name.is_empty());

    let mut cfg = get_test_cfg(0);
    cfg.aggregator_sessions_log_url = "/tmp".to_string();
    let _saver = get_saver(&cfg, Toggle::new("test")).await;
}
