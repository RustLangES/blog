use leptos::{component, view, IntoView};

#[component]
#[must_use]
pub fn CommentIcon(
    #[prop(default = 40)] size: u32,
    #[prop(into, default = "fill-black".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg
            width=size
            height=size
            class=class
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
            role="img"
        >
            <path d="M10 3h4a8 8 0 010 16v3.5c-5-2-12-5-12-11.5a8 8 0 018-8zm2 14h2a6 6 0 000-12h-4a6 6 0 00-6 6c0 3.61 2.462 5.966 8 8.48V17z"></path>
        </svg>
    }
}
