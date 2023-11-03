use std::collections::HashSet;
use std::fs;

use rss::validation::Validate;
use rss::{Category, ChannelBuilder, Item};

use crate::models::article::Article;

pub mod fetch_dev_to;
pub mod fetch_hashnode;

pub fn generate_this_week_feed_rss(articles: &[Article]) {
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

    let min = if articles.len() < 4 {
        0
    } else {
        articles.len() - 4
    };

    let items = articles
        .get(min..)
        .unwrap_or_default()
        .iter()
        .map(|a| a.into())
        .collect::<Vec<Item>>();

    let channel = ChannelBuilder::default()
        .language(Some("es".to_string()))
        .title("Esta Semana en Rust".to_string())
        .description("Revisa que esta pasando en la comunidad de Rust Lang en Español".to_string())
        .link("https://rustlanges.github.io/blog/tags/esta-semana-en-rust.html".to_string())
        .categories(categories)
        .items(items)
        .build();

    channel.validate().unwrap();

    let channel_str = channel.to_string();

    fs::write("./out/this_week_feed.xml", channel_str).unwrap();
    println!("wrote ./out/this_week_feed.xml");
}
