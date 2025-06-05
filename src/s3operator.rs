use aws_config::meta::region::RegionProviderChain;
use aws_config::{defaults, BehaviorVersion, Region};

use aws_credential_types::Credentials;
use aws_sdk_s3::{Client, Config};
use std::error::Error;
use std::panic::{catch_unwind, AssertUnwindSafe};
use tracing::error;

pub struct S3operator {
    pub s3: Option<Client>,
}

impl S3operator {
    pub async fn new(
        region: String,
        access_key: Option<String>,
        secret_key: Option<String>,
        session_token: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut config_loader =
            defaults(BehaviorVersion::latest()).region(Region::new(region.clone()));

        if let (Some(ak), Some(sk)) = (access_key.clone(), secret_key.clone()) {
            let credentials = Credentials::new(
                ak, //
                sk,
                session_token.clone(),
                None,
                "Static",
            );
            config_loader = config_loader
                .credentials_provider(credentials)
                .region(Region::new(region.clone()));
        }

        let conf = config_loader.load().await;

        let s3 = match catch_unwind(
            AssertUnwindSafe(|| Client::new(&conf)), //
        ) {
            Ok(cli) => Some(cli),
            Err(e) => {
                error!("Error get s3 client {:?}", e);
                None
            }
        };

        Ok(S3operator { s3 })
    }
}
