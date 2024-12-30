use leptos::*;
use leptos_mdx::mdx::{Components, Mdx, MdxComponentProps};

use crate::components::mdx::{
    center::{Center, CenterProps},
    youtube::{Youtube, YoutubeProps},
};

#[component]
pub fn MarkdownRender(content: String) -> impl IntoView {
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

    view! {
        <>
            <Mdx source=content components=components />
        </>
    }
}
