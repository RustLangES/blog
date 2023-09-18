use leptos::*;

use github::GithubIcon;
use twitter::TwitterIcon;
use website::WebsiteIcon;

pub mod github;
pub mod twitter;
pub mod website;

#[component]
pub fn StrToIcon(
    #[prop()] v: String,
    #[prop(default = 40)] size: u32,
    #[prop(default = "fill-black")] class: &'static str,
) -> impl IntoView {
    match v.as_str() {
        "github" => view! {
            <>
                <GithubIcon size=size class=class/>
            </>
        },
        "twitter" => view! {
            <>
                <TwitterIcon size=size class=class/>
            </>
        },
        "website" => view! {
            <>
                <WebsiteIcon size=size class=class/>
            </>
        },
        _ => view! { <></> },
    }
}
