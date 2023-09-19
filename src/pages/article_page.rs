use leptos::*;

use crate::{
    components::{blog_content::BlogContent, layout::Layout},
    models::article::Article,
};

#[component]
pub fn ArticlePage(article: Article) -> impl IntoView {
    let title = article.title.clone();
    let description = format!(
        "{} - By @{}",
        article.description.clone(),
        article.github_user.clone()
    );
    view! {
        <>
            <Layout title=title description=description>
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
