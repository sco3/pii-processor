use crate::s3ctx::S3Ctx;
use tracing::{debug, error};

pub struct S3Helper {
    s3ctx: S3Ctx,
}

impl S3Helper {
    pub fn new(s3ctx: S3Ctx) -> Self {
        S3Helper { s3ctx }
    }

    pub fn get_s3ctx(&self) -> &S3Ctx {
        &self.s3ctx
    }

    pub async fn list_buckets(&self) -> Vec<String> {
        let mut found_buckets = Vec::new();
        if let Some(s3) = &self.s3ctx.s3 {
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
    pub async fn get_object(&self, bucket: String, key: String) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(s3) = &self.s3ctx.s3 {
            match s3.get_object().bucket(bucket).key(key).send().await {
                Ok(mut object) => {
                    while let Some(bytes) = object.body.try_next().await.unwrap_or(None) {
                        out.extend_from_slice(&bytes);
                    }
                }
                Err(e) => {
                    error!("Failed to get object: {}", e);
                }
            }
        }
        out
    }
    pub async fn put_object(&self, bucket: String, key: String, data: Vec<u8>) {
        if let Some(s3) = &self.s3ctx.s3 {
            match s3
                .put_object()
                .bucket(bucket.clone())
                .key(key.clone())
                .body(data.into())
                .send()
                .await
            {
                Ok(_) => {
                    debug!("Successfully put object: {} in bucket: {}", key, bucket);
                }
                Err(e) => {
                    error!(
                        "Failed to put object: {} in bucket: {}. Error: {}",
                        key, bucket, e
                    );
                }
            }
        }
    }
}
