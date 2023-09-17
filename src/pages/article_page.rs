
use leptos::*;

use crate::models::article::Article;
use crate::meta::Title;
use crate::components::{layout::Layout, blog_content::BlogContent};

#[component]
pub fn ArticlePage(article: Article) -> impl IntoView {
    let title = article.title.clone();
    view! {
        <Title>{title}</Title>
        <Layout>
            <BlogContent article=article ><p>Children</p></BlogContent>
        </Layout>
    }
}