//! authentication for users

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    client::NoAuthClient,
    item::{Data, Item},
};

use super::error::AuthError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthToken {
    auth_token: String,
}

impl AuthToken {
    pub fn arc(&self) -> Arc<str> {
        Arc::from(self.auth_token.as_str())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Clone)]
pub struct Auth<'a> {
    pub client: NoAuthClient,
    pub user: User<'a>,
}

pub async fn authenticate(auth: Auth<'_>) -> Result<AuthToken, AuthError> {
    let url = format!("{}/authenticate", auth.client.endpoint);

    let data = Data { data: auth.user };
    let json_data = serde_json::to_string(&data)?;

    let authtoken = auth.client.clienttoken.as_ref();

    let response = auth
        .client
        .client
        .post(url)
        .header("content-type", "application/json")
        .header("clienttoken", authtoken)
        .body(json_data)
        .send()
        .await?
        .text()
        .await?;

    let authtoken: Item<AuthToken> =
        serde_json::from_str(&response).map_err(|err| AuthError::Deser {
            serde: err,
            response,
        })?;

    Ok(authtoken.item)
}
