use crate::{
    components::{icons::StrToIcon, markdown_render::MarkdownRender},
    models::article::Article,
};
use leptos::{component, view, CollectView, IntoView};

#[must_use]
#[component]
pub fn CardArticle(article: Article, is_home: bool) -> impl IntoView {
    let article_link = get_link(&article, is_home);
    let description = get_description(&article);

    view! {
        <>
            <li class="group flex flex-col gap-y-1 border border-black p-2 sm:p-6 hover:bg-orange-500 dark:hover:bg-zinc-900/40 bg-orange-100 dark:bg-black/40 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
                <a href=article_link.clone()>
                    <h3 class="text-xl font-semibold">{article.title}</h3>
                </a>
                <p>{article.date_string}</p>
                <div class="text-sm markdown-container">
                    <MarkdownRender content=description />
                </div>
                <div>
                    <span class="pt-4 font-bold">Tags:</span>
                    <TagsList tags=article.tags />
                </div>
                <div class="flex justify-end items-end">
                    <a
                        class="bg-orange-500 group/button hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded flex items-center justify-between gap-2"
                        href=article_link
                    >
                        <span class="group-hover/button:underline">"Leer más"</span>
                        <StrToIcon
                            v="next"
                            class="fill-white group-hover/button:translate-x-1 duration-100"
                            size=16
                        />
                    </a>
                </div>
            </li>
        </>
    }
}

fn get_link(article: &Article, is_home: bool) -> String {
    if is_home {
        format!("./articles/{}.html", article.slug)
    } else {
        format!("./../articles/{}.html", article.slug)
    }
}

fn get_description(article: &Article) -> String {
    if article.description.is_empty() {
        let binding = article.content.clone();
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
    }
}

#[must_use]
#[component]
pub fn TagsList(tags: Option<Vec<String>>) -> impl IntoView {
    let tags = tags.unwrap_or_default();

    view! {
        <ul class="flex gap-1 py-4">
            {tags.into_iter().map(|tag| TagButton(tag.into())).collect_view()}
        </ul>
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn TagButton(tag: String) -> impl IntoView {
    let tag = tag.to_lowercase().replace(' ', "-");

    view! {
        <li class="inline-block text-sm font-bold text-orange-500 hover:text-orange-600">
            <a
                class="inline-block bg-white rounded-md p-1 drop-shadow-sm px-2"
                href=format!("/tags/{}.html", tag)
            >
                {tag}
            </a>
        </li>
    }
}

impl From<String> for TagButtonProps {
    fn from(tag: String) -> Self {
        TagButtonProps { tag }
    }
}

impl From<(Article, bool)> for CardArticleProps {
    fn from((article, is_home): (Article, bool)) -> Self {
        CardArticleProps { article, is_home }
    }
}
