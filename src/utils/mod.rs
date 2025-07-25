use std::{collections::HashSet, fs};

use rss::{validation::Validate, Category, ChannelBuilder, Item};

use crate::models::article::Article;

/// # Panics
/// This can panic if `validate` fails or if `fs::write` fails.
pub fn generate_feed_rss(
    articles: &[Article],
    out_file: &str,
    title: &str,
    description: &str,
    link_path: Option<&str>,
) {
    let categories = articles
        .iter()
        .flat_map(|a| a.tags.clone().unwrap_or_default())
        .collect::<HashSet<String>>();

    let categories = categories
        .iter()
        .map(|c| Category {
            name: c.to_string(),
            domain: None,
        })
        .collect::<Vec<Category>>();

    let items = articles
        .get(..4)
        .unwrap_or_default()
        .iter()
        .map(Into::into)
        .collect::<Vec<Item>>();

    let channel = ChannelBuilder::default()
        .language(Some("es".to_string()))
        .title(title.to_string())
        .description(description.to_string())
        .link(format!(
            "https://blog.rustlang-es.org/{}",
            link_path.unwrap_or_default()
        ))
        .categories(categories)
        .items(items)
        .build();

    channel.validate().unwrap();

    let channel_str = channel.to_string();

    fs::write(out_file, channel_str).unwrap();
    println!("wrote {out_file}");
}
