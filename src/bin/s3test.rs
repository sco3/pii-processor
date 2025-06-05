use dotenv::dotenv;
use ductaper::env_vars::Cfg;
use ductaper::s3ctx::S3Ctx;
use ductaper::s3helper::S3Helper;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cfg = Cfg::from_env();
    let test_bucket = "Dz-bucket-1234".to_string();

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
        assert_eq!(ls.len(), 1);
        if let Some(name) = ls.get(0) {
            assert_eq!(*name, test_bucket)
        } else {
            panic!("Wrong bucket was found!");
        }

        let key = "test-key".to_string();
        let data = b"test data".to_vec();
        s3.put_object(test_bucket.clone(), key.clone(), data.clone())
            .await;
        let out_data = s3.get_object(test_bucket.clone(), key.clone()).await;
        assert_eq!(out_data, data);
    }
}
