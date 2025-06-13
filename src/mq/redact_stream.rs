use crate::config::env_vars::Cfg;
use crate::mq::admin::StreamAdmin;
use crate::util::exit_codes::ExitCode;
use std::process::exit;
use tracing::error;

pub async fn update_redact_stream(
    cfg: &Cfg, //
    admin: StreamAdmin,
    exit_app: bool,
) {
    if let Err(e) = admin
        .update_stream(
            cfg.queue_stream.clone(), //
            vec![StreamAdmin::get_full_subject(
                cfg,
                cfg.redact_subject.clone(),
            )],
        )
        .await
    {
        if exit_app {
            error!("Cannot update stream {}", e);
            exit(ExitCode::NatsError.code());
        } else {
            // for tests
            panic!("Cannot update stream {}", e);
        }
    }
}
