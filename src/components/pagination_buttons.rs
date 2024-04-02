use crate::components::icons::StrToIcon;
use leptos::{component, view, IntoAttribute, IntoView, Show};

#[component]
pub fn PaginationButtons(
    hide: bool,
    current_page: Option<usize>,
    max_page: usize,
) -> impl IntoView {
    let page_number = current_page.unwrap_or(0);

    let show_next_page_button = page_number < max_page || max_page == 0;
    let show_prev_page_button = page_number > 0;

    view! {
        <>

            {if hide {
                view! { <></> }
            } else {
                view! {
                    <>
                        <Show when=move || show_prev_page_button fallback=|| ()>
                            <PreviousPageButton page=current_page/>
                        </Show>
                        <Show when=move || show_next_page_button fallback=|| ()>
                            <NextPageButton page=current_page/>
                        </Show>
                    </>
                }
            }}
        </>
    }
}

#[component]
pub fn PreviousPageButton(page: Option<usize>) -> impl IntoView {
    let page = page.unwrap_or(0);

    let previous_page = if page == 1 {
        "/blog".to_string()
    } else {
        format!("/blog/pages/{}.html", page - 1)
    };

    view! {
        <>
            <a
                href=previous_page
                class="bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded flex items-center justify-between gap-2"
            >
                <StrToIcon v="next" class="fill-white rotate-180" size=16/>
                "Pagina anterior"
            </a>
        </>
    }
}

#[component]
pub fn NextPageButton(page: Option<usize>) -> impl IntoView {
    let page = page.unwrap_or(0);
    let link = format!("/blog/pages/{}.html", page + 1);

    view! {
        <>
            <a
                href=link
                class="bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded flex items-center justify-between gap-2"
            >
                "Siguiente pagina"
                <StrToIcon v="next" class="fill-white" size=16/>
            </a>
        </>
    }
}
