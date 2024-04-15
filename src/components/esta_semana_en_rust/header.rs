use leptos::{component, create_signal, view, IntoAttribute, IntoView, SignalGet, SignalUpdate};

use crate::components::button_link::ButtonLink;

#[component]
pub fn Header() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    view! {
        <header class="border-b border-b-black/20 bg-slate-900 text-white">
            <div class="container mx-auto px-4 flex items-center justify-between flex-col lg:flex-row">
                <div class="flex justify-between w-full lg:w-auto">
                    <a href="/" exact=true class="flex items-center gap-x-4">
                        <img
                            src="https://www.rust-lang.org/static/images/rust-logo-blk.svg"
                            class="max-h-20 rounded-full invert"
                            height="80"
                            width="80"
                            alt="Rust Lang en Español"
                        />
                    </a>
                    <button
                        class="lg:hidden"
                        on:click=move |_| { set_is_open.update(|n| *n = !*n) }
                        aria-label="Menu de opciones"
                    >
                        <span class="w-6 h-1 bg-black block my-4 relative after:absolute after:block after:bg-black after:w-6 after:h-1 after:bottom-2 before:absolute before:block before:bg-black before:w-6 before:h-1 before:-bottom-2"></span>
                    </button>
                </div>
                <nav class=move || {
                    format!(
                        "w-full lg:w-auto pb-10 pt-5 lg:p-0 {}",
                        if is_open.get() { "block" } else { "hidden lg:block" },
                    )
                }>
                    <ul class="flex items-center gap-6 flex-col lg:flex-row lg:items-center ">

                        <li>
                            <a href="https://www.rustlang-es.org/rust-book-es" target="_blank">
                                "El libro"
                            </a>
                        </li>
                        <li>
                            <a href="/aprende">"Aprende"</a>
                        </li>
                        <li>
                            <a href="/comunidad">"Comunidad"</a>
                        </li>
                        <li>
                            <a href="/colaboradores">"Colaboradores"</a>
                        </li>
                        <li>
                            <a href="/blog">"Blog"</a>
                        </li>
                        <li>
                            <ul class="lg:ml-4 flex items-center gap-x-6 ">
                                <li>
                                    <ButtonLink href="https://github.com/RustLangES">
                                        "Github"
                                    </ButtonLink>
                                </li>
                                <li>
                                    <ButtonLink href="https://discord.gg/4ng5HgmaMg">
                                        "Discord"
                                    </ButtonLink>
                                </li>
                            </ul>
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
