pub mod async_component;
pub mod meta;
pub mod render;
pub mod ssg;
pub mod components;
pub mod pages;
pub mod models;

use std::path::Path;

use models::article::Article;
use pages::article_page::ArticlePageProps;
use ssg::Ssg;
use tokio::fs;

use crate::pages::{article_page::ArticlePage, home::Homepage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("./out/articles").await?;

    // initialize the Ssg context
    let ssg = Ssg::new(Path::new("./out"));

    // generate the pages
    ssg.gen("index.html", Homepage).await?;

    let articles = list_articles().await;
    for article in articles {
        ssg.gen(&format!("articles/{}.html", article.slug), || {
            ArticlePage(ArticlePageProps { article })
        })
        .await?;
    }

    Ok(())
}

// this is a fake database call
async fn list_articles() -> Vec<Article> {
    vec![
        Article {
            title: "Bienvenidos a Rust Lang en Español".into(),
            slug: "title-1".into(),
            content: MARKDOWN_SOURCE.into(),
        },
        Article {
            title: "Bienvenidos a Rust Lang en Español".into(),
            slug: "title-2".into(),
            content: MARKDOWN_SOURCE.into(),
        },
        Article {
            title: "Bienvenidos a Rust Lang en Español".into(),
            slug: "title-3".into(),
            content: MARKDOWN_SOURCE.into(),
        },
    ]
}




static MARKDOWN_SOURCE: &str = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl quis
molestie ultricies, **nunc ipsum aliquam nunc**, vitae aliquam nisl nunc eu

## Algo

arcu. Nulla facilisi. Nulla facilisi. Nulla facilisi. Nulla facilisi. Nulla
facilisi. Nulla facilisi. Nulla facilisi. Nulla facilisi. Nulla facilisi. Nulla



## Code
```rust
fn main() {
    println!("hello world !")
}
```

## Math
- $1+1=2$

- $e^{i\pi}+1=0$


$$\int_0^{+\infty}\dfrac{\sin(t)}{t}\,dt=\dfrac{\sqrt{\pi}}{2}$$


## Links and images
![](https://raw.githubusercontent.com/wooorm/markdown-rs/8924580/media/logo-monochromatic.svg?sanitize=true)

for markdown documentation, see [here](https://commonmark.org/help/)

Wikilinks are supported too: [[https://en.wikipedia.org/wiki/Markdown|markdown]]

## Style
| unstyled | styled    |
| :-----:  | ------    |
| bold     | **bold**  |
| italics  | *italics* |
| strike   | ~strike~  |

> Hey, I am a quote !

## Lists
1) one
2) two
3) three

- and
- unorderded
- too

Even todo lists:
- [ ] todo
- [x] done
"#;

