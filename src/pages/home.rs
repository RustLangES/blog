use crate::{
    async_component::Async,
    components::{
        card_article::CardArticle, feature_articles::featured_articles, layout::Layout,
        pagination_buttons::PaginationButtons,
    },
    models::article::Article,
    ARTICLES,
};
use futures::executor::block_on;
use leptos::{component, view, CollectView, IntoView};

async fn fetch_articles() -> Vec<Article> {
    ARTICLES.read().await.clone()
}

#[component]
pub fn Homepage(
    articles: Option<Vec<Article>>,
    show_featured: bool,
    page: Option<usize>,
    max_page: usize,
) -> impl IntoView {
    let mut articles = articles.unwrap_or(block_on(fetch_articles()));

    if show_featured {
        articles = articles.into_iter().take(7).collect();
    }
    let hide_pagination = max_page == 0 && !show_featured;

    view! {
        <Layout slug="https://rustlanges.github.io/preview_concept".to_string()>
            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left mt-2">
                "Blog"
            </h1>
            <p class="text-xl mb-3">
                "Revisa que esta pasando en la comunidad de Rust Lang en Español."
            </p>
            {if show_featured {
                view! { <Async view=featured_articles/> }
            } else {
                view! { <></> }.into_view()
            }}

            <div class="flex w-full flex-row flex-1 items-center mt-6">
                <div class="w-[50%] flex flex-row items-center">
                    <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left my-4">
                        "Artículos"
                    </h1>
                </div>
                <div class="w-[50%] flex justify-end items-center gap-4">
                    <PaginationButtons hide=hide_pagination current_page=page max_page/>
                </div>
            </div>
            <GridOfArticles articles=articles is_home=show_featured/>
        </Layout>
    }
}

#[component]
fn grid_of_articles(articles: Vec<Article>, is_home: bool) -> impl IntoView {
    let mut invalid_tags = vec![
        "esta semana en rust".to_string(),
        "anuncio de la comunidad".to_string(),
    ];

    let articles = if is_home {
        articles
            .into_iter()
            .filter(|article| filter_common_articles(article.clone(), &mut invalid_tags))
            .collect::<Vec<Article>>()
            .into_iter()
    } else {
        articles.into_iter()
    };

    view! {
        <div class="grid sm:grid-cols-2 lg:grid-cols-3 sm:gap-x-8 gap-y-8 pb-5">
            {articles.map(|article| CardArticle((article, is_home).into())).collect_view()}
        </div>
    }
}

pub fn filter_common_articles(article: Article, invalid_tags: &mut Vec<String>) -> bool {
    if let Some(tags) = &article.tags {
        let invalid_tag = invalid_tags.iter().position(|tag| tags.contains(tag));
        if let Some(invalid_tag) = invalid_tag {
            invalid_tags.remove(invalid_tag);
            return false;
        }
        true
    } else {
        true
    }
}
