use crate::{
    async_component::Async,
    components::{
        feature_articles::featured_articles,
        icons::StrToIcon,
        layout::Layout,
        mdx::{
            center::{Center, CenterProps},
            youtube::{Youtube, YoutubeProps},
        },
    },
    models::article::Article,
    ARTICLES,
};
use futures::executor::block_on;
use leptos::{component, view, CollectView, IntoAttribute, IntoView};
use leptos_mdx::mdx::{Components, Mdx, MdxComponentProps};

async fn fetch_articles() -> Vec<Article> {
    ARTICLES.read().await.clone()
}

#[component]
pub fn Homepage(articles: Option<Vec<Article>>, show_featured: bool) -> impl IntoView {
    let articles = articles.unwrap_or(block_on(fetch_articles()));

    view! {
        <Layout slug="https://rustlanges.github.io/preview_concept".to_string()>
            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left mt-2">
                "Blog"
            </h1>
            <p class="text-xl mb-3">
                "Revisa que esta pasando en la comunidad de Rust Lang en Español."
            </p>
            {if show_featured {
                view! { <Async view=featured_articles/> }
            } else {
                view! { <></> }.into_view()
            }}

            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left my-4">
                "Artículos"
            </h1>
            <GridOfArticles articles=articles is_home=show_featured/>
        </Layout>
    }
}

#[component]
fn grid_of_articles(articles: Vec<Article>, is_home: bool) -> impl IntoView {
    let mut invalid_tags = vec![
        "esta semana en rust".to_string(),
        "anuncio de la comunidad".to_string(),
    ];

    let articles = if is_home {
        articles
            .into_iter()
            .filter(|article| filter_common_articles(article.clone(), &mut invalid_tags))
            .collect::<Vec<Article>>()
            .into_iter()
    } else {
        articles.into_iter()
    };

    view! {
        <div class="grid sm:grid-cols-2 lg:grid-cols-3 sm:gap-x-8 gap-y-8 pb-5">
            {articles
                .map(|article| {
                    let description = if article.description.is_empty() {
                        let binding = article.content;
                        let mut content = binding
                            .split('\n')
                            .take(3)
                            .collect::<Vec<&str>>()
                            .join("\n");
                        if content.len() > 190 {
                            content = content[0..190].to_string();
                            content.push_str("...");
                        }
                        content
                    } else {
                        article.description.clone()
                    };
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
                        <li class="group flex flex-col gap-y-1 border border-black p-2 sm:p-6 bg-orange-200 hover:bg-[#fdc686] drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
                            <a href=if is_home {
                                format!("./articles/{}.html", article.slug)
                            } else {
                                format!("./../articles/{}.html", article.slug)
                            }>
                                <h3 class="text-xl font-semibold">{article.title}</h3>
                            </a>
                            <p>{article.date_string}</p>
                            <div class="text-sm markdown-container">
                                <Mdx source=description components=components/>
                            </div>
                            <div>
                                <span class="pt-4 font-bold">Tags:</span>
                                <ul class="flex gap-1 py-4">
                                    {article
                                        .tags
                                        .unwrap_or_default()
                                        .into_iter()
                                        .map(|tag| {
                                            let tag = tag.to_lowercase().replace(' ', "-");
                                            view! {
                                                <>
                                                    <li class="inline-block text-sm font-bold text-orange-500 hover:text-orange-600">
                                                        <a
                                                            class="inline-block bg-white rounded-md p-1 drop-shadow-sm px-2"
                                                            href=if is_home {
                                                                format!("./tags/{}.html", tag)
                                                            } else {
                                                                format!("./../tags/{}.html", tag)
                                                            }
                                                        >

                                                            {tag}
                                                        </a>
                                                    </li>
                                                </>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </div>
                            <div class="flex justify-end items-end">
                                <a
                                    class="bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded flex items-center justify-between gap-2"
                                    href=if is_home {
                                        format!("./articles/{}.html", article.slug)
                                    } else {
                                        format!("./../articles/{}.html", article.slug)
                                    }
                                >

                                    "Leer más"
                                    <StrToIcon v="next" class="fill-white" size=16/>
                                </a>
                            </div>
                        </li>
                    }
                })
                .collect_view()}
        </div>
    }
}

pub fn filter_common_articles(article: Article, invalid_tags: &mut Vec<String>) -> bool {
    if let Some(tags) = &article.tags {
        let invalid_tag = invalid_tags.iter().position(|tag| tags.contains(tag));
        if let Some(invalid_tag) = invalid_tag {
            invalid_tags.remove(invalid_tag);
            return false;
        }
        true
    } else {
        true
    }
}
