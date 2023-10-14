use chrono::Datelike;
use leptos::{component, view, Children, IntoView};

use crate::{
    components::esta_semana_en_rust::header::Header,
    meta::{Head, Html},
};

fn get_year() -> i32 {
    chrono::Utc::now().year()
}

#[component]
// This is a common Layout component that will be used by all pages.
pub fn Layout(
    #[prop(into, default=format!("Blog de Rust Lang en Español {}", get_year()))] title: String,
    #[prop(into, default="Somos una comunidad de Rust hispana, buscamos la promoción del lenguaje de programación Rust.".to_string())]
    description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Html attrs=vec![("lang", "es")] class="bg-slate-800"/>
        <Head>
            <meta charset="utf-8"/>
            <title>{title.clone()}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <meta property="og:title" content=title.clone()/>
            <meta name="description" content=description.clone()/>
            <meta property="og:description" content=description.clone()/>
            <meta
                property="og:site_name"
                content=format!("Blog de Rust Lang en Español {}", get_year())
            />
            <meta property="og:url" content="https://rustlanges.github.io"/>
            <meta property="og:image" content="https://rustlanges.github.io/preview_concept.png"/>
            <meta
                property="twitter:image"
                content="https://rustlanges.github.io/preview_concept.png"
            />
            <meta name="twitter:card" content="summary_large_image"/>
            <meta name="twitter:site" content="@rustlang"/>
            <link rel="icon" href="/LogoSegunMichael-134de58fcd9af94e.ico"/>
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
