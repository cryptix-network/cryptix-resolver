#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("RPC error: {0}")]
    CryptixRpc(#[from] cryptix_wrpc_client::error::Error),

    #[error(transparent)]
    CryptixRpcCore(#[from] cryptix_rpc_core::RpcError),

    #[error(transparent)]
    SparkleRpc(#[from] sparkle_rpc_client::error::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unable to read file: `{0}`, {1}")]
    File(String, std::io::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("Metrics")]
    Metrics,

    #[error("Sync")]
    Sync,

    #[error("Status")]
    Status,

    #[error("Channel send error")]
    ChannelSend,

    #[error("Channel try send error")]
    TryChannelSend,

    #[error(transparent)]
    Encryption(#[from] workflow_encryption::error::Error),

    #[error("Incompatible connection protocol encoding")]
    ConnectionProtocolEncoding,

    #[error("Configuration error")]
    Config(String),

    #[error(transparent)]
    TryFromSlice(#[from] std::array::TryFromSliceError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Could not locate local config")]
    LocalConfigNotFound,

    #[error("Could not locate key file")]
    KeyNotFound,

    #[error("Passwords do not match")]
    PasswordsDoNotMatch,

    #[error("{1}")]
    Http(axum::http::StatusCode, &'static str),

    #[error("Unauthorized")]
    Unauthorized,
}

impl Error {
    pub fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl Error {
    pub fn file<P: AsRef<std::path::Path>>(path: P, err: std::io::Error) -> Self {
        Error::File(path.as_ref().display().to_string(), err)
    }
}

impl Error {
    pub fn config<T: std::fmt::Display>(msg: T) -> Self {
        Error::Config(msg.to_string())
    }
}

impl<T> From<workflow_core::channel::SendError<T>> for Error {
    fn from(_: workflow_core::channel::SendError<T>) -> Self {
        Error::ChannelSend
    }
}

impl<T> From<workflow_core::channel::TrySendError<T>> for Error {
    fn from(_: workflow_core::channel::TrySendError<T>) -> Self {
        Error::TryChannelSend
    }
}
