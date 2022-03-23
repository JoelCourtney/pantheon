use thiserror::Error;
use reqwest::Body;
use serde::de::DeserializeOwned;

pub async fn query<T: DeserializeOwned>(path: impl AsRef<str>, body: impl Into<Body>) -> Result<T, QueryError> {
    use QueryError::*;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "{}/{}",
            seed::window().location().origin().map_err(|e| WindowLocation(e))?,
            path.as_ref()
        ))
        .body(body)
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