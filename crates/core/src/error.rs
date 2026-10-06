use std::path::PathBuf;
use thiserror::Error;

/// Error type duy nhất của app.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("không đọc được file '{path}': {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("không ghi được file '{path}': {source}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("nội dung không hợp lệ: {0}")]
    InvalidContent(String),

    #[error("lỗi serialize JSON: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("lỗi mạng: {0}")]
    Network(String),

    #[error("RPC báo lỗi: {0}")]
    Rpc(String),

    #[error("thiếu cấu hình: {0}")]
    Config(String),
}

pub type AppResult<T> = Result<T, AppError>;