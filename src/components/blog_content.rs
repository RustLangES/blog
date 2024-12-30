use std::collections::HashMap;

use crate::{
    components::{
        icons::StrToIcon,
        mdx::{
            center::{Center, CenterProps},
            youtube::{Youtube, YoutubeProps},
        },
    },
    models::article::Article,
};
use leptos::{component, view, IntoView};
use leptos_mdx::mdx::{Components, Mdx, MdxComponentProps};

#[component]
pub fn BlogContent(
    #[prop()] article: Article,
    #[prop(default = false)] is_html: bool,
) -> impl IntoView {
    let mut components = Components::new();
    let social = article.social.clone().unwrap_or_default();

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

    view! {
        <div class="lg:max-w-5xl my-0 mx-auto w-full">
            <div class="grid justify-items-center lg:grid-cols-sidebar-article gap-4 w-full">
                <aside class="hidden lg:block relative">
                    <div class="sticky top-10 gap-12 left-0 h-screen" id="sidebar">
                        <div class="flex flex-col gap-6 mb-16">
                            <a
                                target="_blank"
                                class="text-sm text-center flex flex-col items-center"
                                href=format!(
                                    "https://twitter.com/intent/tweet?text=https://blog.rustlang-es.org/articles/{}",
                                    article.slug,
                                )
                            >

                                <StrToIcon v="twitter" class="hover:fill-blue-600" />
                                "Compartir en Twitter"
                            </a>
                            <a
                                target="_blank"
                                class="text-sm text-center flex flex-col items-center"
                                href=format!(
                                    "http://www.linkedin.com/shareArticle?mini=true&url=https://blog.rustlang-es.org/articles/{}",
                                    article.slug,
                                )
                            >

                                <StrToIcon v="linkedin" class="hover:fill-blue-600" />
                                "Compartir en Linkedin"
                            </a>
                        </div>
                        <div>
                            <a
                                class="text-sm text-center flex flex-col items-center"
                                href="#giscus"
                            >
                                <StrToIcon v="comment" class="hover:fill-orange-600" />
                                "Ver comentarios"
                            </a>
                        </div>
                    </div>
                </aside>
                <div class="pb-8 lg:max-w-5xl w-full">
                    {ArticleHeader(ArticleHeaderProps {
                        has_author: article.has_author(),
                        title: article.title,
                        github_user: article.github_user,
                        author: article.author,
                        social,
                        tags: article.tags.unwrap_or_default(),
                        date_string: article.date_string,
                    })}
                    <div class="flex w-full border border-black p-6 bg-orange-100 dark:bg-black/40 markdown-container prose max-w-none dark:text-[#e2cea9] dark:prose-invert">
                        {if is_html {
                            view! {
                                <>
                                    <div
                                        class="prose dark:prose-invert max-w-none"
                                        inner_html=article.content
                                    ></div>
                                </>
                            }
                        } else {
                            view! {
                                <>
                                    <Mdx source=article.content components=components />
                                </>
                            }
                        }}

                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ArticleHeader(
    #[prop()] title: String,
    #[prop()] github_user: Option<String>,
    #[prop()] author: Option<String>,
    #[prop()] has_author: bool,
    #[prop()] social: HashMap<String, String>,
    #[prop()] tags: Vec<String>,
    #[prop()] date_string: Option<String>,
) -> impl IntoView {
    view! {
        <>
            <div>
                <h1 class="font-semibold font-work-sans text-xl md:text-3xl text-center lg:text-left">
                    {title.clone()}
                </h1>
                <div class="flex flex-row text-sm items-center flex-wrap">
                    {if github_user.is_some() {
                        view! {
                            <>
                                <img
                                    class="inline-block w-10 h-10 p-[.15rem] border-yellow-800 border-2 rounded-full"
                                    src=format!(
                                        "https://avatars.githubusercontent.com/{}",
                                        github_user.clone().unwrap(),
                                    )
                                />
                            </>
                        }
                    } else {
                        view! { <></> }
                    }}
                    {if has_author {
                        view! {
                            <>
                                <span class="font-semibold m-2">{author}</span>
                            </>
                        }
                    } else {
                        view! { <></> }
                    }}
                    {if tags.is_empty() {
                        view! {
                            <>
                                <span>"habla sobre:"</span>
                            </>
                        }
                    } else {
                        view! { <></> }
                    }}
                    {tags
                        .iter()
                        .map(|tag| {
                            let tag = tag.to_lowercase().replace(' ', "-");
                            view! {
                                <a
                                    class="p-1 m-1 border-2  rounded-full
                                    text-sm font-bold text-orange-500 hover:text-orange-600 bg-white drop-shadow-sm
                                    "
                                    href=format!("/tags/{}.html", tag)
                                >
                                    {tag}
                                </a>
                            }
                        })
                        .collect::<Vec<_>>()}
                    <div class="flex flex-row flex-wrap items-center gap-2">
                        {if !social.is_empty() {
                            view! {
                                <>
                                    <hr class="h-[0.875rem] w-px bg-gray-700 border-0" />
                                </>
                            }
                        } else {
                            view! { <></> }
                        }}
                        {social
                            .iter()
                            .map(|(net, url)| {
                                view! {
                                    <a target="_blank" href=url>
                                        <StrToIcon v=net.to_string() size=15 />
                                    </a>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
            <span class="text-gray-400 text-sm items-center">{date_string}</span>
        </>
    }
}
