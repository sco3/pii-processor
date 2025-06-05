use aws_config::{BehaviorVersion, ConfigLoader, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::Builder as S3ConfigBuilder;
use aws_sdk_s3::Client;
use std::error::Error;

pub struct S3Ctx {
    pub s3: Option<Client>,
    pub bucket: String,
}

impl S3Ctx {
    pub async fn new(
        bucket: String,
        region: String,
        access_key: Option<String>,
        secret_key: Option<String>,
        session_token: Option<String>,
        endpoint_url: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let region = Region::new(region);
        let mut loader = ConfigLoader::default()
            .behavior_version(BehaviorVersion::latest())
            .region(region.clone());

        // If static credentials are provided, apply them
        if let (Some(ak), Some(sk)) = (access_key.clone(), secret_key.clone()) {
            let credentials = Credentials::new(ak, sk, session_token, None, "Static");
            loader = loader.credentials_provider(credentials);
        }

        // Optional custom endpoint (e.g., MinIO)
        if let Some(url) = endpoint_url.clone() {
            loader = loader.endpoint_url(url);
        }

        // Load the shared AWS config
        let shared_config = loader.load().await;

        // Customize S3 client config with force_path_style for MinIO
        let s3_config = S3ConfigBuilder::from(&shared_config)
            .force_path_style(true)
            .build();

        let s3 = Some(Client::from_conf(s3_config));

        Ok(S3Ctx { bucket, s3 })
    }
}
