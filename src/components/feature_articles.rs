use leptos::{component, view, IntoAttribute, IntoView};
use leptos_mdx::mdx::{Components, MdxComponentProps};

use crate::{
    components::{
        icons::StrToIcon,
        mdx::{
            center::{Center, CenterProps},
            youtube::{Youtube, YoutubeProps},
        },
    },
    models::article::Article,
    ARTICLES,
};
use leptos_mdx::mdx::Mdx;

pub async fn featured_articles() -> impl IntoView {
    let articles = ARTICLES.read().await.clone();
    let _invalid_tags = [
        "esta semana en rust".to_string(),
        "anuncio de la comunidad".to_string(),
    ];
    // Take the first article with the tag "esta semana en rust"
    let esta_semana_en_rust = articles
        .clone()
        .into_iter()
        .filter(|article| filter_article_by_tag(article.clone(), "esta semana en rust".to_string()))
        .take(1)
        .collect::<Vec<Article>>();
    let esta_semana_en_rust = esta_semana_en_rust.first().unwrap().to_owned();
    let anuncio_de_la_comunidad = articles
        .into_iter()
        .filter(|article| {
            filter_article_by_tag(article.clone(), "anuncio de la comunidad".to_string())
        })
        .take(1)
        .collect::<Vec<Article>>();

    let anuncio_de_la_comunidad = anuncio_de_la_comunidad.first().unwrap().to_owned();

    let mut components2 = Components::new();
    components2.add_props(
        "youtube".to_string(),
        Youtube,
        |props: MdxComponentProps| {
            let video_id = props.attributes.get("video").unwrap().clone();
            YoutubeProps {
                video: video_id.unwrap(),
            }
        },
    );
    components2.add_props("center".to_string(), Center, |props: MdxComponentProps| {
        CenterProps {
            children: props.children,
        }
    });
    view! {
        <div class="w-full grid lg:grid-cols-divided gap-7">
            <EstaSemanaEnRustCard article=esta_semana_en_rust/>

            <AnuncioDeLaComunidadCard article=anuncio_de_la_comunidad/>
        </div>
    }
}

#[must_use]
pub fn filter_article_by_tag(article: Article, tag: String) -> bool {
    if let Some(tags) = &article.tags {
        tags.contains(&tag)
    } else {
        false
    }
}

#[component]
pub fn EstaSemanaEnRustCard(article: Article) -> impl IntoView {
    let mut components = Components::new();
    components.add_props(
        "youtube".to_string(),
        Youtube,
        |props: MdxComponentProps| {
            let video_id = props.attributes.get("video").unwrap().clone();
            YoutubeProps {
                video: video_id.unwrap(),
            }
        },
    );
    components.add_props("center".to_string(), Center, |props: MdxComponentProps| {
        CenterProps {
            children: props.children,
        }
    });

    let description = article.description.clone();

    view! {
        <div class="rounded-md shadow-md bg-white h-[20rem] md:min-h-[15rem] bg-opacity-70 relative overflow-clip">
            <div class="bg-anuncios h-full w-full bg-opacity-70"></div>
            // Big Header of the card with capital letter and the number of the week of the article
            <div class="absolute backdrop-blur flex flex-row bg-black/50 text-white w-full h-full items-center z-10 top-0 left-0 p-5">
                <div class="w-full h-full flex flex-col gap-5 lg:pr-10 md:pt-10">
                    <div class="w-full items-center justify-between">
                        <h2 class="lg:leading-3lh text-2xl lg:text-[3.5rem] font-semibold font-work-sans tracking-widest max-w-[24ch] text-balance">
                            "Esta semana en Rust"
                            <span class="md:hidden text-2xl lg:text-[3.5rem] font-semibold font-work-sans tracking-widest max-w-[24ch] text-balance">
                                # {article.number_of_week.unwrap()}
                            </span>
                        </h2>
                    </div>
                    <div class="text-lg lg:text-2xl lg:max-w-[700px] text-balance">
                        <Mdx components=components source=description/>
                    </div>
                    <div>
                        <a
                            class="bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded flex items-center text-xl justify-between md:hidden"
                            href=format!("./articles/{}.html", article.slug)
                        >
                            "Leer más"
                            <StrToIcon v="next" class="fill-white" size=20/>
                        </a>
                    </div>
                    <div class="absolute bottom-2 min-h-[2rem]">
                        <p class="text-lg lg:text-2xl">{article.date_string}</p>
                    </div>
                </div>
                <div class="hidden md:flex">
                    <div class="bg-black rounded-full w-40 h-40 md:text-half flex items-center justify-center mb-10">
                        # {article.number_of_week.unwrap()}
                    </div>
                </div>
            </div>
            <div class="absolute -bottom-1 -right-1 z-10 hidden md:block">
                <a
                    class="bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded flex items-center text-xl lg:w-[10rem] justify-between"
                    href=format!("./articles/{}.html", article.slug)
                >
                    "Leer más"
                    <StrToIcon v="next" class="fill-white" size=20/>
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn AnuncioDeLaComunidadCard(article: Article) -> impl IntoView {
    let mut components = Components::new();
    components.add_props(
        "youtube".to_string(),
        Youtube,
        |props: MdxComponentProps| {
            let video_id = props.attributes.get("video").unwrap().clone();
            YoutubeProps {
                video: video_id.unwrap(),
            }
        },
    );
    components.add_props("center".to_string(), Center, |props: MdxComponentProps| {
        CenterProps {
            children: props.children,
        }
    });

    let description = article.description.clone();

    view! {
        <>
            <div class="flex flex-col gap-2">
                <p class="text-1xl tracking-[0.2rem] font-bold">Anuncio:</p>
                <div class="bg-slate-700 rounded-md text-white h-full font-fira-sans p-4 flex flex-col justify-between">
                    <h2 class="text-2xl font-semibold font-work-sans">{article.title.clone()}</h2>
                    <p class="text-white/40">{article.date_string}</p>
                    <div class="text-white/50">
                        <Mdx components=components source=description/>
                    </div>
                    <div>
                        <a
                            class="bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded flex items-center text-xl justify-between"
                            href=format!("./articles/{}.html", article.slug)
                        >
                            "Leer más"
                            <StrToIcon v="next" class="fill-white" size=20/>
                        </a>
                    </div>
                </div>
            </div>
        </>
    }
}
