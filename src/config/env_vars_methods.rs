use crate::config::env_vars::Cfg;
use serde::Deserialize;
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
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Ok(true),
        "false" | "0" | "no" | "n" => Ok(false),
        _ => Err(serde::de::Error::custom(format!("Invalid boolean: {s}"))),
    }
}
