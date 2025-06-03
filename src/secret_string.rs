use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize)]
#[serde(from = "String")]
pub struct SecretString {
    pub value: String,
}

impl SecretString {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn get_string(&self) -> &String {
        &self.value
    }
}

impl From<String> for SecretString {
    fn from(value: String) -> Self {
        SecretString { value }
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
