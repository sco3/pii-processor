use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize)]
pub struct SecretString {
    value: String,
}

impl SecretString {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn get_string(&self) -> &String {
        &self.value
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked = if self.value.len() > 4 {
            format!("{}****", &self.value[..4])
        } else {
            format!("{}****", self.value)
        };
        write!(f, "{}", masked)
    }
}
