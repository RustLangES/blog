use leptos::{component, view, IntoAttribute, IntoView};

#[component]
pub fn Youtube(video: String) -> impl IntoView {
    view! {
        <div class="layout">
            <iframe
                width="560"
                height="315"
                src=format!("https://www.youtube.com/embed/{video}?si=mvuVOHvxBTxYOujJ")
                title="YouTube video player"
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                allowfullscreen
            ></iframe>
        </div>
    }
}
