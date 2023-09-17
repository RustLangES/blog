use crate::{async_component::Async, components::layout::Layout, list_articles};
use leptos::*;
use leptos_markdown::Markdown;

#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <Layout>
            <h1>
                Blog post
            </h1>
            <Async view=list_of_articles/>
        </Layout>
    }
}

async fn list_of_articles() -> impl IntoView {
    let articles = list_articles().await.unwrap_or_default();

    view! {
        <div class="grid sm:grid-cols-2 lg:grid-cols-3 sm:gap-x-8 gap-y-8">
            {articles.into_iter().map(|article| {
                let binding = article.content.to_string().clone();

                let description = binding.split("\n").take(3).collect::<Vec<&str>>().join("\n");
                view! {
                    <li class="group flex flex-col gap-y-6 border border-black p-2 sm:p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
                        <a href={format!("./articles/{}.html", article.slug)}>
                            <h3 class="text-xl font-semibold">{article.title}</h3>
                        </a>
                        <p class="text-sm">
                            <Markdown src=description.to_string()></Markdown>
                        </p>
                    </li>
                }
            }).collect_view()}
        </div>
    }
}
