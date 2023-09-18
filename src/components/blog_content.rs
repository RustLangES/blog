use crate::{components::icons::StrToIcon, models::article::Article};
use leptos::*;
use leptos_mdx::mdx::{Components, Mdx};

#[component]
pub fn BlogContent(#[prop()] article: Article) -> impl IntoView {
    let components = Components::new();

    view! {
        <div class="group flex flex-col gap-y-6 border border-black p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] transition justify-between">
            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left">
                {article.title}
            </h1>
            <div class="flex flex-col">
                <div class="flex flex-row gap-4 text-sm items-center">
                    {if !article.author.is_empty() {
                        view! {
                            <>
                                <h5>{article.author}</h5>
                            </>
                        }
                    } else {
                        view! { <></> }
                    }}
                    {if !article.social.is_empty() {
                        view! {
                            <>
                                <hr class="h-[0.875rem] w-px bg-gray-700 border-0"/>
                            </>
                        }
                    } else {
                        view! { <></> }
                    }}
                    <div class="flex flex-row gap-2 items-center">
                        {article
                            .social
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
                <span class="text-gray-400 text-sm items-center">{article.date}</span>
            </div>
            <div class="markdown-container">
                <Mdx source=article.content.to_string() components=components/>
            </div>
        </div>
    }
}
