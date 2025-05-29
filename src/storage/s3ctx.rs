use aws_config::{BehaviorVersion, ConfigLoader, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Builder as S3ConfigBuilder;
use std::error::Error;
/// s3 login component
pub struct S3Ctx {
    /// s3 client
    pub s3: Option<Client>,
    /// bucket name
    pub bucket: String,
}

impl S3Ctx {
    ///constructor
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if:
    ///
    /// * The AWS SDK configuration cannot be loaded or resolved. This can happen due to:
    ///   * **Invalid AWS credentials:** If environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, etc.),
    ///     shared configuration files (`~/.aws/credentials`), or IAM roles are misconfigured or inaccessible.
    ///   * **Invalid region:** If the provided `region` string is not recognized or causes resolution issues.
    ///   * **Network issues:** Problems reaching AWS metadata services or credential providers during config loading.
    ///   * **Invalid endpoint URL:** If `endpoint_url` is provided but is malformed or causes connection issues during config loading.
    ///
    ///   The underlying error will typically be an `aws_config::Error` or related credential/network errors
    ///   wrapped within `Box<dyn Error>`.
    ///
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

        Ok(S3Ctx { s3, bucket })
    }
}
