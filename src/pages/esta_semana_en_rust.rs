use leptos::*;

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

    let description = format!("{} - By @{}", article.description.clone(), author);
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
                theme="github-dark"
                crossorigin="anonymous"
                async
            ></script>
        </>
    }
}
