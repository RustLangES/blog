use crate::models::article::Article;
use leptos::*;
use leptos_markdown::Markdown;

#[component]
pub fn BlogContent(#[prop()] article: Article, children: Children) -> impl IntoView {
    view! {
        <div
            class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
            style="max-width: 600px;"
        >
            <h1 class="text-xl font-work-sans text-black">{article.title}</h1>
            <Markdown src=article.content.to_string()/>
            {children()}
        </div>
    }
}
