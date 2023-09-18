use crate::models::article::Article;
use leptos::*;
use leptos_markdown::Markdown;

#[component]
pub fn BlogContent(#[prop()] article: Article) -> impl IntoView {
    view! {
        <div class="group flex flex-col gap-y-6 border border-black p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] transition justify-between">
            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left">
                {article.title}
            </h1>
            <Markdown src=article.content.to_string()/>
        </div>
    }
}
