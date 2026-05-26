mod common;

use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use redact::storage::s3ctx::S3Ctx;
use redact::storage::s3helper::S3Helper;
pub use redact::util::logging::init_tracing;
use std::fs::read_to_string;
use std::sync::Arc;

use crate::common::init_cfg::get_test_cfg;
use redact::config::secret_string::SecretString;
use redact::data::session_log_models::SessionLog;
use redact::probe::toggle::Toggle;
use redact::storage::s3_saver::S3Saver;
use redact::storage::saver::Saver;
use redact::storage::saver_factory::get_saver;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::minio;
use tracing::{debug, info};

const MINIOADMIN: &str = "minioadmin";

async fn build_s3_client(host_port: u16) -> Client {
    let endpoint_uri = format!("http://127.0.0.1:{host_port}");

    let region_provider = RegionProviderChain::default_provider() //
        .or_else("us-east-1");

    let creds = Credentials::new(
        MINIOADMIN, MINIOADMIN, None, None, "test", //
    );

    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .endpoint_url(endpoint_uri)
        .credentials_provider(creds)
        .load()
        .await;

    Client::new(&shared_config)
}

#[tokio::test]
async fn test_s3() {
    init_tracing();
    // unsafe {
    //     std::env::remove_var("DOCKER_HOST");
    // }

    let minio = minio::MinIO::default();

    let node = match minio.start().await {
        Ok(node) => node,
        Err(e) => {
            panic!("Failed to start MinIO container: {e}");
        }
    };

    let port = match node.get_host_port_ipv4(9000).await {
        Ok(port) => port,
        Err(e) => {
            panic!("Failed to get host port: {e}");
        }
    };

    info!("Container port {}", port);

    info!(
        "AWS_SECRET_ACCESS_KEY={key} \
        AWS_ACCESS_KEY_ID={key} \
        aws s3api list-buckets \
        --endpoint-url=http://localhost:{port}",
        key = MINIOADMIN,
        port = port
    );
    let test_bucket = "test-bucket".to_string();
    let test_client = build_s3_client(port).await;

    match test_client
        .create_bucket()
        .bucket(test_bucket.clone())
        .send()
        .await
    {
        Ok(_) => info!("Bucket {} created successfully", test_bucket),
        Err(e) => {
            panic!("Bucket create: {test_bucket} {e:?}");
        }
    }
    let endpoint = Some(format!("http://localhost:{port}"));
    let minioadmin = Some(MINIOADMIN.to_string());
    let region = "eu-west-1".to_string();

    if let Ok(s3) = S3Ctx::new(
        test_bucket.clone(),
        region.clone(), //
        minioadmin.clone(),
        minioadmin.clone(),
        None,
        endpoint.clone(),
    )
    .await
    {
        let s3 = S3Helper::new(s3);
        let ls = s3.list_buckets().await;
        assert_eq!(ls.len(), 1);
        if let Some(name) = ls.first() {
            info!("Bucket found: {}", name);
            assert_eq!(*name, test_bucket);
        } else {
            panic!("Wrong bucket was found!");
        }

        let key = "test-key".to_string();
        let in_data = b"test data bytes".to_vec();
        s3.put_object(test_bucket.as_str(), key.clone(), in_data.clone())
            .await;
        let out_data = s3
            .get_object(
                test_bucket.clone(),
                key.clone(), //
            )
            .await
            .unwrap_or_default();
        let s = String::from_utf8_lossy(&out_data);
        info!("Read data: {:?}", s);
        assert_eq!(out_data, in_data);

        assert!(s3.del_object(test_bucket.clone(), key.clone()).await);
        // nonexisting bucket test
        assert!(!s3.del_object("no-bucket".to_string(), key.clone()).await);

        assert!(
            s3.get_object("no-bucket".to_string(), key.clone())
                .await
                .is_none()
        );
        let toggle = Toggle::new("s3");

        let mut cfg = get_test_cfg(0);

        let saver = S3Saver {
            bucket: Arc::from(test_bucket),
            s3helper: Some(s3),
            toggle,
        };
        cfg.aws_s3_endpoint = endpoint;
        cfg.aws_access_key_id = Some(SecretString::new(MINIOADMIN));
        cfg.aws_secret_access_key = Some(SecretString::new(MINIOADMIN));

        cfg.aws_access_token = None;
        //saver.init(&cfg).await;
        let log_str = read_to_string(
            "tests/data/example_new_fields.json", //
        )
        .unwrap();

        if let Ok(log) = serde_json::from_str::<SessionLog>(log_str.as_str()) {
            debug!("Ok");
            saver.save(log, "asdf.json").await;
        }

        let s3toggle = Toggle::new("s3");
        let saver = get_saver(&cfg, s3toggle.clone()).await;
        assert_eq!("s3", saver.get_name());
    }
}
