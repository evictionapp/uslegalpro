//! get the details of a case given a case_tracking_id & authed client, you can get a case_tracking_id via the [query](crate::search::query) method for case_numbers

use crate::{client::AuthedClient, item::Item};

use super::{detail::Detail, error::DetailError};

pub struct Query<'a> {
    pub case_tracking_id: &'a str,
    pub client: AuthedClient,
}

/// get the details of case via a query
pub async fn query(query: Query<'_>) -> Result<Detail, DetailError> {
    let url = format!(
        "{}/case/{}",
        query.client.client.endpoint, query.case_tracking_id,
    );

    let authtoken = query.client.authtoken.as_ref();

    let response = query
        .client
        .client
        .client
        .get(url)
        .header("authtoken", authtoken)
        .send()
        .await?
        .text()
        .await?;

    let item: Item<Detail> = serde_json::from_str(&response).map_err(|err| DetailError::Deser {
        serde: err,
        response,
    })?;

    Ok(item.item)
}
