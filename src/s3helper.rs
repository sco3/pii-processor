//use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, Region};

use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use aws_sdk_s3::Config;
use std::error::Error;
use std::panic::{catch_unwind, AssertUnwindSafe};
use tracing::{error, info};

pub struct S3Helper {
    pub s3: Option<Client>,
    pub bucket: String,
}

impl S3Helper {
    pub async fn new(
        bucket: String,
        region: String,
        access_key: Option<String>,
        secret_key: Option<String>,
        session_token: Option<String>,
        endpoint_url: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut s3_config_builder = Config::builder()
            .behavior_version_latest()
            .region(Region::new(region));

        if let Some(url) = endpoint_url {
            s3_config_builder = s3_config_builder.endpoint_url(url);
        }

        if let (Some(ak), Some(sk)) = (access_key.clone(), secret_key.clone()) {
            let credentials = Credentials::new(
                ak, //
                sk,
                session_token.clone(),
                None,
                "Static",
            );
            s3_config_builder = s3_config_builder.credentials_provider(credentials);
        }
        let s3_conf = s3_config_builder.build();

        let s3 = match catch_unwind(
            AssertUnwindSafe(|| Client::from_conf(s3_conf)), //
        ) {
            Ok(cli) => Some(cli),
            Err(e) => {
                error!("Error get s3 client {:?}", e);
                None
            }
        };

        Ok(S3Helper { bucket, s3 })
    }

    pub async fn list_buckets(&self) {
        if let Some(s3) = &self.s3 {
            let mut buckets = s3.list_buckets().into_paginator().send();
            while let Some(Ok(output)) = buckets.next().await {
                for bucket in output.buckets() {
                    let name = bucket.name().unwrap_or_default();
                    info!("Bucket: {}", name);
                }
            }
        }
    }
}
