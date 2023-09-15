use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetailError {
    /// something went wrong with the request
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    /// something went wrong deserializing the response
    #[error("{serde} - {response}")]
    Deser {
        serde: serde_json::Error,
        response: String,
    },
}
