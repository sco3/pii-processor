use dotenv::dotenv;
use redact::config::env_vars::Cfg;
use redact::storage::s3ctx::S3Ctx;
use redact::storage::s3helper::S3Helper;
use redact::util::logging::init_log;
use tracing::info;

#[tokio::main]
async fn main() {
    init_log(Some("info"));
    dotenv().ok();
    let cfg = Cfg::from_env();
    let test_bucket = "dz-bucket-1234".to_string();

    info!("Configuration: {:?}", cfg);

    if let Ok(s3) = S3Ctx::new(
        test_bucket.clone(),
        "eu-west-1".to_string(), //
        None,
        None,
        None,
        None,
    )
    .await
    {
        let s3 = S3Helper::new(s3);
        let ls = s3.list_buckets().await;

        assert!(!ls.is_empty());
        assert!(
            ls.contains(&test_bucket),
            "Bucket {test_bucket} was not found!"
        );

        let key = "test-key".to_string();
        let expected = b"test data".to_vec();
        s3.put_object(&test_bucket, key.clone(), expected.clone())
            .await;
        let read = s3.get_object(test_bucket.clone(), key.clone()).await;
        assert_eq!(read.unwrap(), expected);
        s3.del_object(test_bucket, key).await;
    }
}
