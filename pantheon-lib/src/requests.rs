use thiserror::Error;
use serde::de::DeserializeOwned;
use crate::shared::Query;

pub async fn send_query<T: DeserializeOwned>(query: Query) -> Result<T, QueryError> {
    use QueryError::*;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "{}/query",
            seed::window().location().origin().map_err(WindowLocation)?
        ))
        .body(bincode::serialize(&query)?)
        .send()
        .await?;
    let bytes = res.bytes().await?;
    Ok(bincode::deserialize(&bytes)?)
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("couldn't get window location origin")]
    WindowLocation(seed::prelude::JsValue),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Bincode(#[from] bincode::Error)
}