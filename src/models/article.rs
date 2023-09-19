use std::collections::HashMap;

use chrono::{Locale, NaiveDate};
use gray_matter::{ParsedEntity, Pod};

pub struct Article {
    pub title: String,
    pub description: String,
    pub author: String,
    pub github_user: String,
    pub slug: String,
    pub content: String,
    pub date: String,
    pub social: HashMap<String, String>,
}

macro_rules! extract_field {
    ($data:expr, $field:expr) => {
        $data
            .get($field)
            .unwrap_or(&Pod::String("".to_string()))
            .as_string()
            .unwrap_or_default()
    };
}

impl From<ParsedEntity> for Article {
    fn from(parsed_entity: ParsedEntity) -> Self {
        let data = parsed_entity
            .data
            .clone()
            .unwrap()
            .as_hashmap()
            .unwrap_or_default();
        let title = extract_field!(data, "title");
        let description = extract_field!(data, "description");
        let author = extract_field!(data, "data");
        let github_user = extract_field!(data, "github_user");
        let slug = extract_field!(data, "slug");
        let content = parsed_entity.content;

        let date = {
            let date_str = extract_field!(data, "date");
            let mut splitted = date_str.split('-');
            let year = splitted
                .next()
                .unwrap_or_default()
                .parse::<i32>()
                .unwrap_or_default();
            let month = splitted
                .next()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default();
            let day = splitted
                .next()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default();
            let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            date.format_localized("%e de %B del %Y", Locale::es_ES)
                .to_string()
        };

        let default_social = Pod::Hash(HashMap::new());
        let social = data
            .get("social")
            .unwrap_or(&default_social)
            .as_hashmap()
            .map(|i| {
                i.iter()
                    .map(|(k, v)| (k.to_string(), v.as_string().unwrap()))
                    .collect::<HashMap<String, String>>()
            })
            .unwrap();

        Self {
            title,
            description,
            author,
            github_user,
            slug,
            content,
            social,
            date: date.to_string(),
        }
    }
}
