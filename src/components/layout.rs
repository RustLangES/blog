use chrono::Datelike;
use leptos::*;

use crate::{
    components::header::Header,
    meta::{Head, Html, Title},
};

#[component]
// This is a common Layout component that will be used by all pages.
pub fn Layout(children: Children) -> impl IntoView {
    let year = chrono::Utc::now().year();

    view! {
        <Html attrs=vec![("lang", "es")]/>
        <Title>{format!("Blog de Rust Lang en Español {year}")}</Title>
        <Head>
            <meta charset="utf-8"/>
            <meta property="og:site_name" content=format!("Blog de Rust Lang en Español {year}")/>
            <meta property="og:title" content=format!("Blog de Rust Lang en Español {year}")/>
            <meta
                property="og:description"
                content="Somos una comunidad de Rust hispana, buscamos la promoción del lenguaje de programación Rust."
            />
            <meta
                name="description"
                content="Somos una comunidad de Rust hispana, buscamos la promoción del lenguaje de programación Rust."
            />
            <meta property="og:url" content="https://rustlanges.github.io"/>
            <meta property="og:image" content="https://rustlanges.github.io/preview_concept.png"/>
            <meta
                property="twitter:image"
                content="https://rustlanges.github.io/preview_concept.png"
            />
            <meta name="twitter:card" content="summary_large_image"/>
            <meta name="twitter:site" content="@rustlang"/>
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
            <main class="container mx-auto py-5">{children()}</main>
        </section>
    }
}
