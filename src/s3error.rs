use aws_sdk_s3::error::SdkError;

/// Returns a shortened version of the AWS SDK error message,
/// removing sensitive information like access keys or request IDs.
pub fn aws_err<E: std::fmt::Debug>(e: &SdkError<E>) -> String {
    let msg = format!("{:?}", e);

    // Markers indicating the start of sensitive data
    const MARKERS: [&str; 2] = ["aws_request_id", "s3_extended_request_id"];

    // Find the earliest marker
    let split_pos = MARKERS.iter().filter_map(|marker| msg.find(marker)).min();

    match split_pos {
        Some(pos) => format!("{}...", &msg[..pos]),
        None => msg,
    }
}
