/// exit codes for application
pub enum ExitCode {
    /// ok
    Success = 0,
    /// probe failed to start
    ProbeError = 1,
    /// s3 failed on start
    S3Error = 2,
    /// nats problem
    NatsError = 3,
    /// prompt read failed
    PromptError = 4,
}

impl ExitCode {
    /// return number representation
    #[must_use]
    pub fn code(self) -> i32 {
        self as i32
    }
}
