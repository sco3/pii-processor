/// s3 bucket checker
pub mod get_bucket;
/// component to write results to local fs
pub mod local_saver;
/// s3 saver - writes result to s3
pub mod s3_saver;
/// s3 connect setup module
pub mod s3ctx;
/// s3 error logging function to hide sensitive secrets
pub mod s3error;
/// s3 utility module - list, save, read
pub mod s3helper;
/// interface (trait) for saving result to s3/fs
pub mod saver;
/// based on url creates saver
pub mod saver_factory;
