use std::fmt;

///structures with process results

#[derive(Debug)]
/// metric for processing
pub struct Metrics {
    /// storage save duration in microsecnods
    pub save_micros: u128,
    /// storage name
    pub save_kind: String,
    // add whatever metric fields you want
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} save time: {} us", self.save_kind, self.save_micros,)
    }
}

#[derive(Debug)]
/// process result return value
pub enum ProcessResult {
    /// ok
    Ok(Metrics),
    /// parsing error
    ParseError(Option<Metrics>),
    /// other erros
    Error(Option<Metrics>),
}
