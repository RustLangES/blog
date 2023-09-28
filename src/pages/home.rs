use crate::{
    async_component::Async,
    components::{
        layout::Layout,
        mdx::{
            center::{Center, CenterProps},
            youtube::{Youtube, YoutubeProps},
        },
    },
    list_articles,
};
use leptos::*;
use leptos_mdx::mdx::{Components, Mdx, MdxComponentProps};

#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <Layout>
            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left my-2">
                "Blog"
            </h1>
            <p class="text-xl">
                "Revisa que esta pasando en la comunidad de Rust Lang en Español."
            </p>
            <Async view=list_of_articles/>
        </Layout>
    }
}

async fn list_of_articles() -> impl IntoView {
    let articles = list_articles().await.unwrap_or_default();

    view! {
        <div class="grid sm:grid-cols-2 lg:grid-cols-3 sm:gap-x-8 gap-y-8 py-5">
            {articles
                .into_iter()
                .map(|article| {
                    let binding = article.content.to_string().clone();
                    let description = binding.split('\n').take(3).collect::<Vec<&str>>().join("\n");
                    let mut components = Components::new();
                    components
                        .add_props(
                            "youtube".to_string(),
                            Youtube,
                            |props: MdxComponentProps| {
                                let video_id = props.attributes.get("video").unwrap().clone();
                                YoutubeProps {
                                    video: video_id.unwrap(),
                                }
                            },
                        );
                    components
                        .add_props(
                            "center".to_string(),
                            Center,
                            |props: MdxComponentProps| {
                                CenterProps {
                                    children: props.children,
                                }
                            },
                        );
                    view! {
                        <li class="group flex flex-col gap-y-1 border border-black p-2 sm:p-6 bg-orange-200 hover:bg-orange-300 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
                            <a href=format!("./articles/{}.html", article.slug)>
                                <h3 class="text-xl font-semibold">{article.title}</h3>
                            </a>
                            <p>{article.date}</p>
                            <div class="text-sm markdown-container">
                                <Mdx source=description.to_string() components=components/>
                            </div>
                            <a
                                class="bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded"
                                href=format!("./articles/{}.html", article.slug)
                            >
                                "Leer más"
                            </a>
                        </li>
                    }
                })
                .collect_view()}
        </div>
    }
}
