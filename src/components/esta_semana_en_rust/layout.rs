use chrono::Datelike;
use leptos::{component, view, Children, IntoView};

use crate::{
    components::esta_semana_en_rust::header::Header,
    meta::{Head, Html},
};

fn get_year() -> i32 {
    chrono::Utc::now().year()
}

// This is a common Layout component that will be used by all pages.
#[component]
#[must_use]
pub fn Layout(
    #[prop(into, default=format!("Blog de Rust Lang en Español {}", get_year()))] title: String,
    #[prop(into, default="Somos una comunidad de Rust hispana, buscamos la promoción del lenguaje de programación Rust.".to_string())]
    slug: String,
    description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Html attrs=vec![("lang", "es")] class="bg-slate-800" />
        <Head>
            <meta charset="utf-8" />
            <title>{title.clone()}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <meta property="og:title" content=title.clone() />
            <meta name="description" content=description.clone() />
            <meta property="og:description" content=description.clone() />
            <meta
                property="og:site_name"
                content=format!("Blog de Rust Lang en Español {}", get_year())
            />
            <meta property="og:url" content="https://rustlang-es.org" />
            <meta
                property="og:image"
                content=format!("https://blog.rustlang-es.org/articles/{slug}.png")
            />
            <meta
                property="twitter:image"
                content=format!("https://blog.rustlang-es.org/articles/{slug}.png")
            />
            <link rel="canonical" href=format!("https://blog.rustlang-es.org/articles/{slug}") />
            <meta name="twitter:card" content="summary_large_image" />
            <meta name="twitter:site" content="@rustlang" />
            <link rel="icon" href="/LogoSegunMichael-134de58fcd9af94e.ico" />
            {if cfg!(debug_assertions) {
                view! { <link rel="stylesheet" href="/output.css" /> }
            } else {
                view! { <link rel="stylesheet" href="https://blog.rustlang-es.org/output.css" /> }
            }}

            <style>
                {"
                body {
                margin: 0 auto;
                }
                "}
            </style>
            <script type="module">
                {"
                const API = 'https://rust-lang-en-espanol-api.shuttleapp.rs';
                let previous_domain = document.referrer || 'Undefined';
                if (previous_domain != 'Undefined') { previous_domain = new URL(previous_domain).host; }
                const urlParams = new URLSearchParams(window.location.search);
                const fromParam = urlParams.get('from');
                if (fromParam != null) previous_domain = fromParam;
                await fetch(API + '/track/count?reference=' + previous_domain, { method: 'POST' });
                "}
            </script>
            <script type="text/javascript">
                (function(c,l,a,r,i,t,y){
                    if ("localhost0.0.0.0::0192.168.0.1192.168.1.1".includes(document.location.hostname)) return;
                    c[a]=c[a]||function(){(c[a].q=c[a].q||[]).push(arguments)};
                    t=l.createElement(r);t.async=1;t.src="https://www.clarity.ms/tag/"+i;
                    y=l.getElementsByTagName(r)[0];y.parentNode.insertBefore(t,y);
                })(window, document, "clarity", "script", "n5t1gk7y5v");
            </script>
        </Head>

        // Async is a component from the async_component module.
        // It will wrap an async function that returns an IntoView.
        <section class="w-full flex flex-col">
            <Header />

            // <Async view=navigation_bar />
            <main class="container mx-auto py-5">{children()}</main>
        </section>
    }
}
