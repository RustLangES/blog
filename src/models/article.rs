use std::collections::HashMap;

use chrono::{Locale, NaiveDate};
use rss::{Category, Guid, Item, Source};
use serde::{Deserialize, Deserializer, Serialize};

use super::{devto_article::DevToArticle, hashnode_article::ArticleFetchedPost};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Article {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub authors: Option<Vec<String>>,
    pub github_user: Option<String>,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub number_of_week: Option<u32>,
    #[serde(
        rename(deserialize = "date"),
        deserialize_with = "string_to_naive_date"
    )]
    pub date: NaiveDate,
    #[serde(default)]
    pub date_string: Option<String>,
    #[serde(default)]
    pub rfc_date: Option<String>,
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

        Self {
            title: devto_article.title,
            description: devto_article.description,
            author: Some(devto_article.user.name),
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
                    format!("https://github.com/{}", devto_article.user.github_username),
                ),
            ])),
            slug: devto_article.slug,
            date_string: Some(
                date_time
                    .format_localized("%e de %B del %Y", Locale::es_ES)
                    .to_string(),
            ),
            content: devto_article.content_html.unwrap_or_default(),

            devto: true,
            tags: Some(devto_article.tag_list),
            ..Default::default()
        }
    }
}

impl From<ArticleFetchedPost> for Article {
    fn from(hashnode_article: ArticleFetchedPost) -> Self {
        let date_time =
            NaiveDate::parse_from_str(&hashnode_article.date_added, "%Y-%m-%dT%H:%M:%S%.fZ")
                .unwrap();

        Self {
            title: hashnode_article.title,
            description: hashnode_article.brief,
            author: Some(hashnode_article.publication.username),
            github_user: hashnode_article
                .publication
                .links
                .github
                .split('/')
                .last()
                .map(std::string::ToString::to_string),
            date: date_time,
            social: Some(HashMap::from([
                (
                    "twitter".to_string(),
                    hashnode_article.publication.links.twitter,
                ),
                (
                    "github".to_string(),
                    hashnode_article.publication.links.github,
                ),
            ])),
            slug: hashnode_article.slug,
            content: hashnode_article.content_markdown,
            date_string: Some(
                date_time
                    .format_localized("%e de %B del %Y", Locale::es_ES)
                    .to_string(),
            ),
            devto: false,
            tags: Some(
                hashnode_article
                    .tags
                    .iter()
                    .map(|tag| tag.name.clone().to_lowercase().replace(' ', "-"))
                    .collect(),
            ),
            ..Default::default()
        }
    }
}

impl From<&Article> for Item {
    fn from(value: &Article) -> Self {
        let link = format!(
            "https://rustlanges.github.io/blog/articles/{}.html",
            value.slug.clone()
        );
        Item {
            title: Some(value.title.clone()),
            link: Some(link),
            description: Some(value.description.clone()),
            author: value.author.clone(),
            categories: value
                .tags
                .clone()
                .map(|c| {
                    c.iter()
                        .map(|c| Category {
                            name: c.to_string(),
                            domain: None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            guid: Some(Guid {
                value: value.slug.clone(),
                permalink: false,
            }),
            pub_date: value.rfc_date.clone(),
            source: Some(Source {
                url: "https://github.com/RustLangES/blog".to_string(),
                title: Some("Repositorio del Blog".to_string()),
            }),
            content: None,
            ..Default::default()
        }
    }
}

impl Article {
    #[must_use]
    pub fn has_author(&self) -> bool {
        if let Some(author) = &self.author {
            !author.is_empty()
        } else {
            false
        }
    }
}
