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
        <>
            <Title>{title}</Title>
            <Layout>
                <BlogContent article=article/>
            </Layout>
            <script
                src="https://utteranc.es/client.js"
                repo="RustLangES/blog"
                issue-term="title"
                label="comentarios 💬"
                theme="github-light"
                crossorigin="anonymous"
                async
            ></script>
        </>
    }
}
