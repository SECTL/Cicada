use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("token expired")]
    TokenExpired,

    #[error("unauthorized")]
    Unauthorized,

    #[error("server error: {0}")]
    ServerError(String),

    #[error("PKCE error: {0}")]
    PKCEError(String),
}
