//use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, ConfigLoader, Region};

use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use std::error::Error;

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

        Ok(S3Helper { bucket, s3 })
    }

    pub async fn list_buckets(&self) {
        if let Some(s3) = &self.s3 {
            //println!("s3: {:?}", s3);
            let mut buckets = s3.list_buckets().into_paginator().send();
            println!("b: {:?}", buckets);
            while let Some(Ok(output)) = buckets.next().await {
                for bucket in output.buckets() {
                    let name = bucket.name().unwrap_or_default();
                    println!("Bucket: {:?}", name);
                }
            }
        }
    }
    pub async fn list_buckets2(&self) {
        if let Some(s3) = &self.s3 {
            // Access the underlying client's config to get the region
            if let Some(region) = s3.config().region() {
                println!("S3 Client configured for region: {}", region.as_ref());
            } else {
                println!("S3 Client region is not explicitly set in its configuration.");
            }

            println!("Attempting to list S3 buckets...");
            let mut paginator = s3.list_buckets().into_paginator().send();

            let mut found_buckets = false;
            while let Some(result) = paginator.next().await {
                match result {
                    Ok(output) => {
                        for bucket in output.buckets() {
                            let name = bucket.name().unwrap_or_default();
                            println!("Found Bucket: {}", name);
                            found_buckets = true;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching a page of buckets: {:?}", e);
                        break;
                    }
                }
            }
            if !found_buckets {
                println!("No buckets found or an error occurred during listing.");
            } else {
                println!("Finished listing S3 buckets.");
            }
        } else {
            eprintln!("S3 client is not initialized in S3Helper.");
        }
    }
}
