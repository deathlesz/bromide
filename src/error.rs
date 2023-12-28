#![allow(clippy::enum_variant_names)]

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to access config file: {0}")]
    FailedToLoadConfig(#[from] std::io::Error),
    #[error("failed to deserialize config: {0}")]
    FailedToDeserializeConfig(#[from] toml::de::Error),
    #[error("failed to serialize config: {0}")]
    FailedToSerializeConfig(#[from] toml::ser::Error),
}
