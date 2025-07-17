use crate::config::list_env::mask;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

/// String wrapper that masks sensitive values in logs and serialization.
#[derive(Clone, Deserialize)]
#[serde(from = "String")]
pub struct SecretString {
    /// The actual sensitive string value
    pub value: String,
}

impl SecretString {
    /// Creates a new `SecretString` from a string slice.
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    /// Returns the unmasked string value (use with caution).
    #[must_use]
    pub fn get_string(&self) -> String {
        self.value.clone()
    }
}

impl From<String> for SecretString {
    /// Converts a String into a `SecretString`.
    fn from(value: String) -> Self {
        SecretString { value }
    }
}

impl fmt::Debug for SecretString {
    /// Formats the string for debugging, showing a masked version.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked = mask(&self.value);
        write!(f, "{masked}")
    }
}

impl Serialize for SecretString {
    /// Serializes the string in masked format.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{self:?}"))
    }
}
