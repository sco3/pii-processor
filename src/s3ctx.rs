//use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, ConfigLoader, Region};

use aws_credential_types::Credentials;
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
        let mut loader = ConfigLoader::default()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(region.clone()));

        if let (Some(ak), Some(sk)) = (access_key.clone(), secret_key.clone()) {
            let credentials = Credentials::new(
                ak, //
                sk,
                session_token.clone(),
                None,
                "Static",
            );
            loader = loader.credentials_provider(credentials);
        }
        if let Some(url) = endpoint_url {
            loader = loader.endpoint_url(url);
        }

        let s3 = Some(Client::new(&loader.load().await));

        Ok(S3Ctx { bucket, s3 })
    }

}
