use crate::config::env_vars::Cfg;
use crate::mq::stream_admin::StreamAdmin;
use crate::util::exit_codes::ExitCode;
use std::process::exit;
use tracing::error;
/// updates nats stream
pub async fn update_redact_stream(admin: &StreamAdmin, cfg: &Cfg) {
    let subject = StreamAdmin::get_full_subject(
        cfg, //
        &cfg.redact_subject,
    );
    if let Err(e) = admin
        .check_stream(
            cfg.queue_stream.clone(), //
            vec![subject],
        )
        .await
    {
        error!("Update stream: {}", e);
        exit(ExitCode::NatsError.code());
    }
}
