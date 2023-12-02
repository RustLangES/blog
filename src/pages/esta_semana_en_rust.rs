use leptos::{component, view, IntoView};

use crate::{
    components::esta_semana_en_rust::{blog_content::BlogContent, layout::Layout},
    models::article::Article,
};

#[component]
pub fn EstaSemanaEnRust(article: Article) -> impl IntoView {
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
                <BlogContent article=article/>
            </Layout>
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
                data-theme="dark_tritanopia"
                data-lang="es"
                crossorigin="anonymous"
                async
            ></script>
        </>
    }
}
