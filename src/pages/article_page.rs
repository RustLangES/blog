use leptos::{component, view, IntoView};

use crate::{
    components::{blog_content::BlogContent, layout::Layout},
    models::article::Article,
};

#[component]
pub fn ArticlePage(article: Article) -> impl IntoView {
    let title = article.title.clone();

    let author = if let Some(github_user) = article.github_user.clone() {
        github_user
    } else {
        article.author.clone().unwrap_or_default()
    };

    let description = format!("{} - By @{}", article.description, author);
    view! {
        <>
            <Layout title=title description=description slug=article.slug.clone()>
                <BlogContent is_html=article.devto article=article/>
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
