pub enum ExitCode {
    Success = 0,
    ProbeError = 1,
    S3Error = 2,
    NatsError = 3,
}
impl ExitCode {
    pub fn code(self) -> i32 {
        self as i32
    }
}
