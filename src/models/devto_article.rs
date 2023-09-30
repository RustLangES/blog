use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type DevToArticles = Vec<DevToArticle>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevToArticle {
    #[serde(rename = "type_of")]
    pub type_of: String,
    pub id: u32,
    pub title: String,
    pub description: String,
    #[serde(rename = "readable_publish_date")]
    pub readable_publish_date: String,
    pub slug: String,
    pub path: String,
    pub url: String,
    #[serde(rename = "comments_count")]
    pub comments_count: i64,
    #[serde(rename = "public_reactions_count")]
    pub public_reactions_count: i64,
    #[serde(rename = "collection_id")]
    pub collection_id: Value,
    #[serde(rename = "published_timestamp")]
    pub published_timestamp: String,
    #[serde(rename = "positive_reactions_count")]
    pub positive_reactions_count: i64,
    #[serde(rename = "cover_image")]
    pub cover_image: Value,
    #[serde(rename = "social_image")]
    pub social_image: String,
    #[serde(rename = "canonical_url")]
    pub canonical_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "edited_at")]
    pub edited_at: Value,
    #[serde(rename = "crossposted_at")]
    pub crossposted_at: Value,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "last_comment_at")]
    pub last_comment_at: String,
    #[serde(rename = "reading_time_minutes")]
    pub reading_time_minutes: i64,
    #[serde(rename = "tag_list")]
    pub tag_list: Vec<String>,
    pub tags: String,
    pub user: User,
    #[serde(skip_deserializing)]
    pub content: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub username: String,
    #[serde(rename = "twitter_username")]
    pub twitter_username: String,
    #[serde(rename = "github_username")]
    pub github_username: String,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(rename = "website_url")]
    pub website_url: String,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "profile_image_90")]
    pub profile_image_90: String,
}
