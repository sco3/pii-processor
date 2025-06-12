use dotenv::dotenv;
use ductaper::env_vars::Cfg;
use ductaper::logging::init_log;
use ductaper::storage::s3ctx::S3Ctx;
use ductaper::storage::s3helper::S3Helper;
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
        Some(cfg.aws_access_key_id.unwrap().get_string()),
        Some(cfg.aws_secret_access_key.unwrap().get_string()),
        None,
        None,
    )
    .await
    {
        let s3 = S3Helper::new(s3);
        let ls = s3.list_buckets().await;

        assert!(!ls.is_empty());
        if !ls.contains(&test_bucket) {
            panic!("Bucket {} was not found!", test_bucket);
        }

        let key = "test-key".to_string();
        let expected = b"test data".to_vec();
        s3.put_object(test_bucket.clone(), key.clone(), expected.clone())
            .await;
        let read = s3.get_object(test_bucket.clone(), key.clone()).await;
        assert_eq!(read.unwrap(), expected);
        s3.del_object(test_bucket, key).await;
    }
}
