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
            <Layout
                title=title
                description=description
                slug=format!("blog/articles/{}", article.slug.clone())
            >
                <BlogContent is_html=article.devto article=article/>
            </Layout>
            <div id="giscus" class="giscus max-w-5xl mx-auto"></div>
            <script
                src="https://giscus.app/client.js"
                data-repo="RustLangES/blog"
                data-repo-id="R_kgDOKUmlrg"
                data-category="Comentarios de Artículos"
                data-category-id="DIC_kwDOKUmlrs4CbcBv"
                data-mapping="title"
                data-strict="0"
                data-reactions-enabled="1"
                data-emit-metadata="0"
                data-input-position="bottom"
                data-theme="light"
                data-lang="es"
                crossorigin="anonymous"
                async
            ></script>
        </>
    }
}
