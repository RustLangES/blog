use leptos::*;

use crate::meta::{Html, Title, Head};
use crate::components::header::Header;

#[component]
// This is a common Layout component that will be used by all pages.
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        // Html and Title are components from the meta module. They will be
        // rendered to the <head> of the page.
        <Html attrs=vec![("lang", "es")] />
        <Title>"RustLang Es | Blog"</Title>
        <Head>
            <meta charset="utf-8" />
            {if cfg!(debug_assertions) {
                view!{ <link rel="stylesheet" href="/output.css" /> }
            }else{
                view!{ <link rel="stylesheet" href="https://rustlanges.github.io/blog/output.css" /> }
            }}
            <style>{r#"
                body {
                    margin: 0 auto;
                }
            "#}</style>
        </Head>

        // Async is a component from the async_component module.
        // It will wrap an async function that returns an IntoView.
        <section class="w-full flex flex-col">
            <Header></Header>
            
            <main class="container mx-auto">
                {children()}
                // <Async view=navigation_bar />
            </main>
        </section>
    }
}