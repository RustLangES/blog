use std::collections::HashMap;

use chrono::{Locale, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize};

use super::devto_article::DevToArticle;

#[derive(Serialize, Deserialize)]
pub struct Article {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub github_user: Option<String>,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub content: String,

    #[serde(
        rename(deserialize = "date"),
        deserialize_with = "string_to_naive_date"
    )]
    pub date: NaiveDate,
    #[serde(default)]
    pub date_string: Option<String>,
    pub social: Option<HashMap<String, String>>,
    #[serde(default)]
    pub devto: bool,
}

fn string_to_naive_date<'de, D>(de: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
    D::Error: serde::de::Error,
{
    let date_str: String = Deserialize::deserialize(de)?;
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap_or_default();

    Ok(date)
}

impl From<DevToArticle> for Article {
    fn from(devto_article: DevToArticle) -> Self {
        let date_time =
            NaiveDate::parse_from_str(&devto_article.published_at, "%Y-%m-%dT%H:%M:%SZ").unwrap();

        Article {
            title: devto_article.title,
            description: devto_article.description,
            author: devto_article.user.name,
            github_user: Some(devto_article.user.github_username.clone()),
            date: date_time,
            social: Some(HashMap::from([
                (
                    "twitter".to_string(),
                    format!(
                        "https://twitter.com/{}",
                        devto_article.user.twitter_username
                    ),
                ),
                (
                    "github".to_string(),
                    format!(
                        "https://github.com/{}",
                        devto_article.user.github_username.clone()
                    ),
                ),
            ])),
            slug: devto_article.slug,
            date_string: Some(
                date_time
                    .format_localized("%e de %B del %Y", Locale::es_ES)
                    .to_string(),
            ),
            content: devto_article.content.unwrap_or_default(),
            devto: true,
        }
    }
}
