pub mod async_component;
pub mod meta;
pub mod render;
pub mod ssg;
pub mod components;
pub mod pages;
pub mod models;

use std::{path::Path, fs};

use gray_matter::{Matter, engine::YAML};
use models::article::Article;
use pages::article_page::ArticlePageProps;
use ssg::Ssg;

use crate::pages::{article_page::ArticlePage, home::Homepage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let articles = list_articles().await?;

    tokio::fs::create_dir_all("./out/articles").await?;

    // initialize the Ssg context
    let ssg = Ssg::new(Path::new("./out"));

    // generate the pages
    ssg.gen("index.html", Homepage).await?;

    for article in articles {
        ssg.gen(&format!("articles/{}.html", article.slug), || {
            ArticlePage(ArticlePageProps { article })
        })
        .await?;
    }

    Ok(())
}

async fn list_articles() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let paths = fs::read_dir("./articles")?;

    let mut articles = Vec::with_capacity(10);

    for path in paths {
        let file = path?.path();
        let algo = fs::read_to_string(file.clone())?;
        let matter =  Matter::<YAML>::new();
        let parsed_entity = matter.parse(&algo);
        let mut article: Article = parsed_entity.into();
        if article.slug == "" {
            // path without extension
            let filename_without_extension = file.file_stem().unwrap().to_str().unwrap();
            article.slug = filename_without_extension.to_string();
        }
        articles.push(article);
    }


    Ok(articles)
}
