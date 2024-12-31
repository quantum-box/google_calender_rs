use thiserror::Error;

#[derive(Error, Debug)]
pub enum GCalError {
    #[error("HTTPリクエストエラー: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSONパースエラー: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("設定エラー: {0}")]
    ConfigError(String),

    #[error("認証エラー: {0}")]
    AuthError(String),

    #[error("その他エラー: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, GCalError>;
