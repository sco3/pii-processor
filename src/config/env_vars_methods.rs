use crate::config::env_vars::Cfg;
use serde::de::{Error, Visitor};

use std::fmt;
use tracing::info;
impl Cfg {
    /// Load configuration from environment variables
    ///
    /// # Panics
    /// Panics if required environment variables are missing or invalid
    #[must_use]
    pub fn from_env() -> Self {
        envy::from_env::<Cfg>().unwrap_or_else(|err| {
            panic!("Failed to load configuration : {err}");
        })
    }

    /// Pretty print the current configuration
    ///
    /// Logs all configuration values at INFO level, with sensitive fields redacted
    pub fn pretty(&self) {
        if let Ok(value) = serde_json::to_value(self) {
            if let Some(obj) = value.as_object() {
                for (key, val) in obj {
                    info!("{} : {}", key.to_uppercase(), val);
                }
            }
        }
    }
}

/// Deserialize a boolean from various string representations
///
/// # Arguments
/// * `deserializer` - The Serde deserializer
///
/// # Returns
/// `Result<bool, D::Error>` - Parsed boolean or error
///
/// # Supported formats
/// True values: "true", "1", "yes", "y"
/// False values: "false", "0", "no", "n"
pub fn de_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct BoolVisitor;

    impl Visitor<'_> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean or a string representing a boolean")
        }

        // Handle direct boolean values (JSON `true` or `false`)
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }

        // Handle string values ("true", "false", "1", "0", "yes", "no", etc.)
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match v.to_lowercase().as_str() {
                "true" | "1" | "yes" | "y" => Ok(true),
                "false" | "0" | "no" | "n" => Ok(false),
                _ => Err(E::custom(format!("Invalid boolean string: {v}"))),
            }
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}
