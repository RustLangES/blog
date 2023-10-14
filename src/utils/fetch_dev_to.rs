use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::value::Value;

use crate::models::devto_article::DevToArticles;

pub async fn fetch_dev_to() -> Result<DevToArticles, reqwest::Error> {
    let url = "https://dev.to/api/articles?tag=rust,%20spanish";
    let mut headers = HeaderMap::new();
    headers.append("Accept", HeaderValue::from_static("application/json"));

    let client = Client::builder()
        .user_agent("RustLangEs")
        .default_headers(headers)
        .build()?;

    let mut resp = client
        .get(url)
        .send()
        .await?
        .json::<DevToArticles>()
        .await?;

    for article in resp.iter_mut() {
        let article_complete = get_article_by_id(article.id, &client).await?;
        let Value::String(content) = article_complete.get("body_markdown").unwrap() else {
            continue;
        };
        article.content = Some(content.to_string());
    }

    Ok(resp)
}

pub async fn get_article_by_id(
    id: u32,
    client: &Client,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let url = format!("https://dev.to/api/articles/{}", id);
    let resp = client
        .get(&url)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;

    Ok(resp)
}
