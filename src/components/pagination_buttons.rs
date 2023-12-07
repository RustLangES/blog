use crate::components::icons::StrToIcon;
use leptos::{component, view, IntoAttribute, IntoView};

#[component]
pub fn PaginationButtons(
    hide: bool,
    current_page: Option<usize>,
    max_page: usize,
) -> impl IntoView {
    view! {
        <>

            {if hide {
                view! { <></> }
            } else {
                let page_number = current_page.unwrap_or(0);
                let hide_next_page_button = page_number >= max_page && max_page != 0;
                view! {
                    <>
                        <PreviousPageButton page=current_page/>
                        <NextPageButton page=current_page max_page hide=hide_next_page_button/>
                    </>
                }
            }}
        </>
    }
}

#[component]
pub fn PreviousPageButton(page: Option<usize>) -> impl IntoView {
    let page = page.unwrap_or(0);

    if page == 0 {
        return view! { <></> };
    }

    let previous_page = if page == 1 {
        "..".to_string()
    } else {
        format!("../pages/{}.html", page - 1)
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
pub fn NextPageButton(page: Option<usize>, max_page: usize, hide: bool) -> impl IntoView {
    if hide {
        return view! { <></> };
    }

    let page = page.unwrap_or(0);

    let link = if max_page == 0 {
        format!("./pages/{}.html", page + 1)
    } else {
        format!("../pages/{}.html", page + 1)
    };

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
