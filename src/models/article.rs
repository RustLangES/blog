use gray_matter::{ParsedEntity, Pod};

pub struct Article {
    pub title: String,
    pub slug: String,
    pub content: String,
}

impl From<ParsedEntity> for Article {
    fn from(parsed_entity: ParsedEntity) -> Self {
        let data = parsed_entity
            .data
            .clone()
            .unwrap()
            .as_hashmap()
            .unwrap_or_default();
        let title = data
            .get("title")
            .unwrap_or(&Pod::String("".to_string()))
            .as_string()
            .unwrap_or_default();
        let slug = data
            .get("slug")
            .unwrap_or(&Pod::String("".to_string()))
            .as_string()
            .unwrap_or_default();
        let content = parsed_entity.content;

        Self {
            title,
            slug,
            content,
        }
    }
}
