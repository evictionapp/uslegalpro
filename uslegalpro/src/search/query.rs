//! contains the "query" method on case previews, used to get case tracking ids for case numbers.

use crate::{client::AuthedClient, item::Items};

use super::{error::PreviewError, preview::Preview};

/// a case query, requires an authed client + case_number & jurisdiction
pub struct Query<'a> {
    pub client: AuthedClient,
    pub case_number: &'a str,
    pub jurisdiction: &'a str,
}

pub async fn query(query: Query<'_>) -> Result<Vec<Preview>, PreviewError> {
    let url = format!("{}/search_case", query.client.client.endpoint);

    let jurisdiction = query.jurisdiction;
    let case_number = query.case_number;
    let authtoken = query.client.authtoken.as_ref();

    let response = query
        .client
        .client
        .client
        .get(url)
        .query(&[("jurisdiction", jurisdiction), ("case_number", case_number)])
        .header("authtoken", authtoken)
        .send()
        .await?
        .text()
        .await?;

    let items: Items<Preview> =
        serde_json::from_str(&response).map_err(|err| PreviewError::Deser {
            serde: err,
            response,
        })?;

    Ok(items.items)
}
