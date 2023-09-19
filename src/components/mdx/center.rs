use leptos::*;

#[component]
pub fn Center(children: Children) -> impl IntoView {
    view! { <div class="mx-auto">{children()}</div> }
}
