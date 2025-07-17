use url::Url;

/// Extracts the S3 bucket name from a string using a URL parser.
///
/// This function treats the S3 URL as a generic URL and extracts the 'host'
/// component, which corresponds to the bucket name in "s3://<bucket>" format.
/// It handles cases where the URL might include a path.
///
/// Arguments:
/// * `s3_url`: A string slice representing the S3 URL.
///
/// Returns:
/// * `Option<String>`:
///     - `Some(bucket_name)` if the URL is valid, its scheme is "s3", and a host (bucket) is found.
///     - `None` otherwise.
#[must_use]
pub fn get_bucket(s3_url: &str) -> Option<String> {
    // 1. Parse the input string into a Url object.
    //    This returns a Result, as parsing can fail if the URL is malformed.
    let parsed_url = Url::parse(s3_url);

    match parsed_url {
        Ok(url) => {
            // 2. Check if the scheme is "s3".
            if url.scheme() == "s3" {
                // 3. Extract the host component.
                //    For s3://<bucket>/path, the <bucket> is the host.
                //    `host_str()` returns an `Option<&str>`.
                if let Some(bucket_str) = url.host_str() {
                    // 4. Ensure the host string is not empty.
                    if !bucket_str.is_empty() {
                        return Some(bucket_str.to_string());
                    }
                }
            }
            None // Not an s3 scheme, or no host found
        }
        Err(_) => {
            // Parsing failed, so it's not a valid URL or not in the expected format.
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_s3_url_with_object() {
        let url = "s3://my-test-bucket";
        assert_eq!(get_bucket(url), Some("my-test-bucket".to_string()));
    }
}
