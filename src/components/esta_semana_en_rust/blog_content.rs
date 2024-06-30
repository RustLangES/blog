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
pub fn BlogContent(#[prop()] article: Article) -> impl IntoView {
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
        <div class=" container mx-auto max-w-3xl">
            <div class=" group flex flex-col gap-y-6 border p-6 bg-slate-800 drop-shadow-[0_0_0_rgba(0,0,0)] transition justify-between text-white border-transparent">
                <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left">
                    {article.title.clone()}
                </h1>
                <div class="flex flex-col">
                    <div class="flex flex-row gap-4 text-sm items-center">
                        {if !article.has_author() {
                            view! {
                                <>
                                    <h5>{article.author}</h5>
                                </>
                            }
                        } else {
                            view! { <></> }
                        }}
                        {if !social.is_empty() {
                            view! {
                                <>
                                    <hr class="h-[0.875rem] w-px bg-gray-700 border-0"/>
                                </>
                            }
                        } else {
                            view! { <></> }
                        }}
                        <div class="flex flex-row gap-2 items-center">
                            {social
                                .iter()
                                .map(|(net, url)| {
                                    view! {
                                        <a target="_blank" href=url>
                                            <StrToIcon v=net.to_string() size=15/>
                                        </a>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>
                    <span class="text-gray-400 text-sm items-center">{article.date_string}</span>
                </div>
                <div class="markdown-container prose max-w-none prose-invert">
                    <Mdx source=article.content components=components/>
                </div>
            </div>
        </div>
    }
}
