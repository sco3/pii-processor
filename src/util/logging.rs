use std::sync::Once;
use tracing::{Level, error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub static LOG_INIT: Once = Once::new();

#[allow(dead_code)]
pub fn init_log(level_str: Option<&str>) {
    LOG_INIT.call_once(|| {
        let mut level: Level = Level::INFO;
        if let Some(s) = level_str {
            match s.to_uppercase().parse::<Level>() {
                Ok(l) => {
                    level = l;
                }
                Err(e) => {
                    error!("Unknown level: {} {}", s, e);
                }
            }
        }
        let mut filter = EnvFilter::new(level.to_string());
        filter = filter
            .add_directive(
                "async_nats=warn"
                    .parse()
                    .expect("Failed to parse directive"),
            )
            .add_directive(
                "hyper_util=warn"
                    .parse()
                    .expect("Failed to parse directive"),
            )
            .add_directive(
                "S3=warn".parse().expect("Failed to parse directive"), //
            )
            .add_directive(
                "aws_sdk_s3=warn"
                    .parse()
                    .expect("Failed to parse directive"), //
            )
            .add_directive(
                "aws_smithy_runtime=warn"
                    .parse()
                    .expect("Failed to parse directive"),
            )
            .add_directive(
                "aws_smithy_runtime_api=warn"
                    .parse()
                    .expect("Failed to parse directive"),
            )
            .add_directive(
                "aws_config=warn"
                    .parse()
                    .expect("Failed to parse directive"),
            )
            .add_directive(
                "aws_types=warn" //
                    .parse()
                    .expect("Failed to parse directive"),
            )
            .add_directive(
                "aws_runtime=warn" //
                    .parse()
                    .expect("Failed to parse directive"),
            )
            .add_directive(
                "aws_smithy_http_client=warn" //
                    .parse()
                    .expect("Failed to parse directive"),
            );

        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .with_env_filter(filter)
            .with_test_writer()
            .with_file(true)
            .with_line_number(true)
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            println!("Sorry. Tracing already initialized: {}", e);
        }
        info!(
            "Logging level: {} from configuration: {:?}).",
            level, level_str
        );
    });
}

pub fn init_tracing() {
    init_log(Some(Level::DEBUG.to_string().as_str()));
}
