use leptos::*;

use crate::{
    components::{blog_content::BlogContent, layout::Layout},
    meta::Title,
    models::article::Article,
};

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
