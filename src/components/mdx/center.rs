use leptos::{component, view, Children, IntoView};

#[component]
#[must_use]
pub fn Center(children: Children) -> impl IntoView {
    view! { <div class="mx-auto">{children()}</div> }
}
