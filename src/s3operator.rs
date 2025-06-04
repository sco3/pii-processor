pub struct S3operator {
    pub region: String,
}

impl S3operator {
    pub fn new(region: String) -> Self {
        S3operator { region }
    }
}
