use std::collections::HashMap;

use serde_json::value::Value;

use crate::models::devto_article::DevToArticles;

pub async fn fetch_dev_to() -> Result<DevToArticles, reqwest::Error> {
    let url = "https://dev.to/api/articles?tag=rust,%20spanish";
    let client = reqwest::Client::new();
    let mut resp = client
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "RustLangEs")
        .send()
        .await?
        .json::<DevToArticles>()
        .await?;

    for article in resp.iter_mut() {
        let article_complete = get_article_by_id(article.id).await?;
        article.content = Some(article_complete.get("body_markdown").unwrap().to_string());
    }

    Ok(resp)
}

pub async fn get_article_by_id(id: u32) -> Result<HashMap<String, Value>, reqwest::Error> {
    let url = format!("https://dev.to/api/articles/{}", id);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "RustLangEs")
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    println!("{:#?}", resp);
    Ok(resp)
}
