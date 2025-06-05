//use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, ConfigLoader, Region};

use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use std::error::Error;
use tracing::{debug, error};

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

    pub async fn list_buckets(&self) -> Vec<String> {
        let mut found_buckets = Vec::new();
        if let Some(s3) = &self.s3 {
            let mut buckets = s3.list_buckets().into_paginator().send();
            while let Some(result) = buckets.next().await {
                match result {
                    Ok(output) => {
                        for bucket in output.buckets() {
                            let name = bucket.name().unwrap_or_default();
                            debug!("Bucket: {}", name);
                            found_buckets.push(name.to_string());
                        }
                    }
                    Err(e) => {
                        error!("Sdk error: {}", e);
                        break;
                    }
                }
            }
        }
        found_buckets
    }
}
