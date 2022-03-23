use anyhow::Result;
use reqwest::Body;
use serde::de::DeserializeOwned;

pub async fn query<T: DeserializeOwned>(path: impl AsRef<str>, body: impl Into<Body>) -> Result<T> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "{}/{}",
            seed::window().location().origin().unwrap(),
            path.as_ref()
        ))
        .body(body)
        .send()
        .await?;
    let bytes = res.bytes().await?;
    Ok(bincode::deserialize(&bytes)?)
}
