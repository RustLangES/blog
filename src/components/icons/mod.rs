use leptos::{component, view, IntoView};

use comments::CommentIcon;
use github::GithubIcon;
use linkedin::LinkedinIcon;
use logo_rust_page::LogoRustPageIcon;
use next::NextIcon;
use twitter::TwitterIcon;
use website::WebsiteIcon;

pub mod comments;
pub mod github;
pub mod linkedin;
pub mod logo_rust_page;
pub mod next;
pub mod twitter;
pub mod website;

#[component]
pub fn StrToIcon(
    #[prop(into)] v: String,
    #[prop(default = 40)] size: u32,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let class = "fill-black dark:fill-white ".to_owned() + class;

    match v.as_str() {
        "github" => view! {
            <>
                <GithubIcon size=size class=class />
            </>
        },
        "twitter" => view! {
            <>
                <TwitterIcon size=size class=class />
            </>
        },
        "website" => view! {
            <>
                <WebsiteIcon size=size class=class />
            </>
        },
        "next" => view! {
            <>
                <NextIcon size=size class=class />
            </>
        },
        "linkedin" => view! {
            <>
                <LinkedinIcon size=size class=class />
            </>
        },
        "comment" => view! {
            <>
                <CommentIcon size=size class=class />
            </>
        },
        "rust" => view! {
            <>
                <LogoRustPageIcon size=size class=class />
            </>
        },
        _ => view! { <></> },
    }
}
