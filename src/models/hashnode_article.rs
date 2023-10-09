use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HashnodeResponse {
    pub data: Option<Data>,
    pub errors: Option<Vec<serde_json::Value>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub publication: Publication,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub posts: Vec<Post>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleFetched {
    pub data: Option<ArticleFetchedData>,
    pub errors: Option<Vec<serde_json::Value>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleFetchedData {
    pub post: ArticleFetchedPost,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleFetchedPost {
    pub slug: String,
    pub title: String,
    pub tags: Vec<Tag>,
    pub date_added: String,
    pub content_markdown: String,
    pub brief: String,
    pub publication: ArticleFetchedPublication,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleFetchedPublication {
    pub links: Links,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub hashnode: String,
    pub website: String,
    pub github: String,
    pub twitter: String,
}
