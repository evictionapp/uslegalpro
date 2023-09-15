use thiserror::Error;

#[derive(Debug, Error)]
pub enum PreviewError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{serde} - {response}")]
    Deser {
        serde: serde_json::Error,
        response: String,
    },
}
