pub mod async_component;
pub mod components;
pub mod meta;
pub mod models;
pub mod pages;
pub mod render;
pub mod ssg;
pub mod utils;

use std::{
    fs::{self, ReadDir},
    path::Path,
};

use futures::future::join_all;
use futures_concurrency::prelude::*;
use gray_matter::{engine::YAML, Matter};
use models::article::Article;
use once_cell::sync::Lazy;
use pages::{
    article_page::ArticlePageProps,
    esta_semana_en_rust::{EstaSemanaEnRust, EstaSemanaEnRustProps},
    home::HomepageProps,
};
use ssg::Ssg;
use tokio::sync::RwLock;
use utils::{fetch_dev_to::fetch_dev_to, fetch_hashnode::fetch_hashnode, generate_feed_rss};

use crate::pages::{article_page::ArticlePage, home::Homepage};

pub static ARTICLES: Lazy<RwLock<Vec<Article>>> =
    Lazy::new(|| RwLock::new(Vec::with_capacity(1010)));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let articles = list_articles().await?;
    ARTICLES.write().await.extend(articles.clone()); // Set the articles in the ARTICLES static variable
    let out = Path::new("./out/blog");
    if !out.exists() {
        std::fs::create_dir_all(out).expect("Cannot create 'out' directory");
    }
    let ssg = Ssg::new(out);

    // generate the pages
    (
        generate_homepage(&ssg),
        generate_post_pages(articles.clone(), &ssg),
        generate_pages(articles.clone(), &ssg),
        generate_esta_semana_en_rust(articles.clone(), &ssg),
        generate_tag_pages(articles.clone(), &ssg),
    )
        .try_join()
        .await?;

    Ok(())
}

async fn generate_homepage<'a>(ssg: &Ssg<'a>) -> Result<(), Box<dyn std::error::Error>> {
    ssg.gen("index.html".to_owned(), || {
        Homepage(HomepageProps {
            articles: None,
            show_featured: true,
            page: None,
            max_page: 0,
        })
    })
    .await?;

    Ok(())
}

async fn generate_esta_semana_en_rust<'a>(
    articles: Vec<Article>,
    ssg: &Ssg<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    let articles = articles
        .into_iter()
        .filter(|article| article.number_of_week.is_some())
        .collect::<Vec<Article>>();

    generate_feed_rss(
        &articles,
        "./out/blog/this_week_feed.xml",
        "Esta Semana en Rust",
        "Revisa que esta pasando en la comunidad de Rust Lang en Español",
        Some("tags/esta-semana-en-rust.html"),
    );

    let generate_articles = articles.into_iter().map(|article| {
        if article.number_of_week.is_some() {
            ssg.gen(format!("articles/{}.html", article.slug), || {
                EstaSemanaEnRust(EstaSemanaEnRustProps { article })
            })
        } else {
            panic!("articles without number_of_week should be generated in generate_post_pages")
        }
    });

    join_all(generate_articles).await;

    Ok(())
}

async fn generate_post_pages<'a>(
    articles: Vec<Article>,
    ssg: &Ssg<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::create_dir_all("./out/blog/articles").await?;

    let articles = articles
        .clone()
        .into_iter()
        .filter(|a| a.number_of_week.is_none())
        .collect::<Vec<Article>>();

    generate_feed_rss(
        &articles,
        "./out/blog/feed.xml",
        "Blog de RustLangES",
        "Enterate del mejor contenido en Español sobre Rust",
        None,
    );

    let generate_articles = articles.into_iter().map(|article| {
        if article.number_of_week.is_none() {
            ssg.gen(format!("articles/{}.html", article.slug), move || {
                ArticlePage(ArticlePageProps { article })
            })
        } else {
            panic!(
                "articles with number_of_week should be generated in generate_esta_semana_en_rust"
            )
        }
    });

    join_all(generate_articles).await;

    Ok(())
}

async fn generate_tag_pages<'a>(
    articles: Vec<Article>,
    ssg: &Ssg<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::create_dir_all("./out/blog/tags").await?;

    let tags = articles
        .iter()
        .filter_map(|article| article.tags.clone())
        .flatten()
        .map(|tag| {
            let articles_to_show = articles
                .clone()
                .into_iter()
                .filter(|article| {
                    if let Some(tags) = article.tags.clone() {
                        tags.contains(&tag)
                    } else {
                        false
                    }
                })
                .collect::<Vec<Article>>();

            let tag = tag.to_lowercase().replace(' ', "-");

            ssg.gen(format!("tags/{tag}.html"), || {
                Homepage(HomepageProps {
                    articles: Some(articles_to_show),
                    show_featured: false,
                    page: None,
                    max_page: 0,
                })
            })
        });

    join_all(tags).await;

    Ok(())
}

async fn list_articles() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let mut articles = Vec::with_capacity(10);
    let article_folder = fs::read_dir("./articles")?;
    articles.append(&mut posts_from_folder(article_folder)?);

    let esta_semana_en_rust_folder = fs::read_dir("./esta_semana_en_rust")?;
    articles.append(&mut posts_from_folder(esta_semana_en_rust_folder)?);

    if !cfg!(debug_assertions) {
        let dev_to_articles = fetch_dev_to().await?;
        let hashnode_articles = fetch_hashnode().await?;

        articles.append(
            &mut dev_to_articles
                .into_iter()
                .map(Article::from)
                .collect::<Vec<Article>>(),
        );

        articles.append(
            &mut hashnode_articles
                .into_iter()
                .map(Article::from)
                .collect::<Vec<Article>>(),
        );
    }

    articles.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(articles)
}

fn posts_from_folder(paths: ReadDir) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let mut articles = Vec::with_capacity(10);

    for path in paths {
        let file = path?.path();
        let algo = fs::read_to_string(file.clone())?;
        let matter = Matter::<YAML>::new();
        let Some(parsed_entity) = matter.parse_with_struct(&algo) else {
            println!("Error parsing file: {file:?}");
            continue;
        };
        let content = parsed_entity.content.clone();
        let mut article: Article = parsed_entity.data;
        article.content = content;
        if article.slug.is_empty() {
            // path without extension
            let filename_without_extension = file.file_stem().unwrap().to_str().unwrap();
            article.slug = filename_without_extension.to_string();
        }
        if article.date_string.is_none() {
            article.date_string = Some(
                article
                    .date
                    .format_localized("%e de %B del %Y", chrono::Locale::es_ES)
                    .to_string(),
            );
        }
        articles.push(article);
    }
    Ok(articles)
}

async fn generate_pages<'a>(
    mut articles: Vec<Article>,
    ssg: &Ssg<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::create_dir_all("./out/blog/pages").await?;

    if let Some(last_this_week_in_rust) = articles.iter().position(|a| a.number_of_week.is_some()) {
        articles.remove(last_this_week_in_rust);
    }
    if let Some(announce_position) = articles.iter().position(|a| {
        a.tags
            .as_ref()
            .unwrap()
            .contains(&"anuncio de la comunidad".to_string())
    }) {
        articles.remove(announce_position);
    }
    let max_pages = articles.len() / 6;
    let mut articles = articles.chunks(6).collect::<Vec<_>>();
    articles.remove(0);
    let articles = articles.to_vec();

    let generate_articles = articles
        .iter()
        .enumerate()
        .map(|(index, articles_to_show)| {
            let articles_to_show = articles_to_show.to_vec();
            ssg.gen(format!("pages/{}.html", index + 2), move || {
                Homepage(HomepageProps {
                    articles: Some(articles_to_show),
                    show_featured: false,
                    page: Some(index + 2),
                    max_page: max_pages,
                })
            })
        });

    join_all(generate_articles).await;

    Ok(())
}
