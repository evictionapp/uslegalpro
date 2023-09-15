//! There are two types of clients
//! No authed & Authed
//! No authed client can only access apis that don't require authentication
//! Authed clients can access any api
//! Authed clients can be constructed from no authed clients via the [authenticate](crate::auth::authenticate) method

use std::sync::Arc;

use crate::auth::authenticate::AuthToken;

/// Create a client that isn't authenticated only requires a client token and an endpoint, along with a reqwest client
/// You can use the "State" enum in the [state](crate::state) module to quickly create an endpoint to common states.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct NoAuthClient {
    pub client: reqwest::Client,
    pub clienttoken: Arc<str>,
    pub endpoint: Arc<str>,
}

impl NoAuthClient {
    pub fn new(client: reqwest::Client, clienttoken: &str, endpoint: &str) -> Self {
        Self {
            client,
            clienttoken: Arc::from(clienttoken),
            endpoint: Arc::from(endpoint),
        }
    }

    /// convert the no authed client into an authed one via an authtoken.
    /// you can get an authetoken via the "authentication" method [here](crate::auth::authenticate)
    pub fn into_authed_client(self, authtoken: &AuthToken) -> AuthedClient {
        AuthedClient {
            client: self,
            authtoken: authtoken.arc(),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct AuthedClient {
    pub client: NoAuthClient,
    pub authtoken: Arc<str>,
}
