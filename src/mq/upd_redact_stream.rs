use crate::config::env_vars::Cfg;
use crate::mq::admin::StreamAdmin;
use crate::util::exit_codes::ExitCode;
use std::process::exit;
use tracing::error;

pub async fn update_redact_stream(admin: &StreamAdmin, cfg: &Cfg) {
    let subject = StreamAdmin::get_full_subject(
        &cfg, //
        cfg.redact_subject.clone(),
    );
    if let Err(e) = admin
        .update_stream(
            cfg.queue_stream.clone(), //
            vec![subject],
        )
        .await
    {
        error!("Update stream: {}", e);
        exit(ExitCode::NatsError.code());
    }
}
