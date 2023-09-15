use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("{0}")]
    Ser(#[from] serde_json::Error),

    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{serde} - {response}")]
    Deser {
        serde: serde_json::Error,
        response: String,
    },
}
