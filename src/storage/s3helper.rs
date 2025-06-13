use crate::storage::s3ctx::S3Ctx;
use crate::storage::s3error::aws_err;
use tracing::{debug, error, info};
pub struct S3Helper {
    s3ctx: S3Ctx,
}

impl S3Helper {
    pub fn new(s3ctx: S3Ctx) -> Self {
        S3Helper { s3ctx }
    }

    // pub fn get_s3ctx(&self) -> &S3Ctx {
    //     &self.s3ctx
    // }

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
                        error!("Failed list buckets: {}", aws_err(&e));
                        break;
                    }
                }
            }
        }
        found_buckets
    }
    pub async fn get_object(&self, bucket: String, key: String) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        if let Some(s3) = &self.s3ctx.s3 {
            match s3.get_object().bucket(bucket).key(key).send().await {
                Ok(mut object) => {
                    while let Some(bytes) = object.body.try_next().await.unwrap_or(None) {
                        out.extend_from_slice(&bytes);
                    }
                }
                Err(e) => {
                    error!("Failed to get object: {}", aws_err(&e));
                    return None;
                }
            }
        }
        Some(out)
    }
    pub async fn put_object(&self, bucket: String, key: String, data: Vec<u8>) -> bool {
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
                    info!("Successfully put object: {} in bucket: {}", key, bucket);
                    return true;
                }
                Err(e) => {
                    error!(
                        "Failed to put object: {} in bucket: {}. Error: {}",
                        key,
                        bucket,
                        aws_err(&e)
                    );
                    return false;
                }
            }
        }
        false
    }

    pub async fn del_object(&self, bucket: String, key: String) -> bool {
        if let Some(s3) = &self.s3ctx.s3 {
            match s3
                .delete_object()
                .bucket(bucket.clone())
                .key(key.clone())
                .send()
                .await
            {
                Ok(_) => {
                    info!("Successfully deleted: {} bucket: {}", key, bucket);
                    return true;
                }
                Err(e) => {
                    error!(
                        "Failed to delete: {} from bucket: {}. Error: {}",
                        key,
                        bucket,
                        aws_err(&e)
                    );
                }
            }
        }
        false
    }
}
