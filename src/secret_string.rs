use crate::config::list_env::mask;
use serde::{Deserialize, Serialize, Serializer};
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

    pub fn get_string(&self) -> String {
        self.value.clone()
    }
}

impl From<String> for SecretString {
    fn from(value: String) -> Self {
        SecretString { value }
    }
}
impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked = mask(self.value.clone());
        write!(f, "{}", masked)
    }
}

impl Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 1. Get the debug (masked) string representation of `self`.
        //    This calls the `fmt::Debug` implementation you've already provided.
        let masked_string_representation = format!("{:?}", self);

        // 2. Serialize this `masked_string_representation` as a JSON string.
        serializer.serialize_str(&masked_string_representation)
    }
}
