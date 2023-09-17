use leptos::*;

use crate::{
    components::header::Header,
    meta::{Head, Html, Title},
};

#[component]
// This is a common Layout component that will be used by all pages.
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <Html attrs=vec![("lang", "es")]/>
        <Title>"RustLang Es | Blog"</Title>
        <Head>
            <meta charset="utf-8"/>
            {if cfg!(debug_assertions) {
                view! { <link rel="stylesheet" href="/output.css"/> }
            } else {
                view! {
                    <link rel="stylesheet" href="https://rustlanges.github.io/blog/output.css"/>
                }
            }}

            <style>
                {"
                body {
                margin: 0 auto;
                }
                "}
            </style>
        </Head>

        // Async is a component from the async_component module.
        // It will wrap an async function that returns an IntoView.
        <section class="w-full flex flex-col">
            <Header/>

            // <Async view=navigation_bar />
            <main class="container mx-auto">{children()}</main>
        </section>
    }
}
