use std::{cell::RefCell, collections::HashMap, rc::Rc};

use leptos::{ssr::render_to_string, *};

#[component]
pub fn Html(#[prop(into)] mut attrs: Attrs) -> impl IntoView {
    let ctx = expect_context::<ShellCtx>();
    let mut class = Attrs::from(vec![("class", "bg-orange-200")]);
    ctx.body_attrs.borrow_mut().append(&mut class);
    ctx.html_attrs.borrow_mut().append(&mut attrs);
    ()
}

#[component]
pub fn Head(children: Children) -> impl IntoView {
    let ctx = expect_context::<ShellCtx>();
    ctx.head_els.borrow_mut().push(children());
    ()
}

#[component]
pub fn Title(children: Children) -> impl IntoView {
    view! {
        <Dedup key="title">
            <title>{children()}</title>
        </Dedup>
    }
}

#[component(transparent)]
pub fn Dedup(#[prop(into)] key: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<ShellCtx>();
    let mut map = ctx.deduped_head_els.borrow_mut();
    if !map.contains_key(&key) {
        map.insert(key, children());
    }
    ()
}

#[derive(Clone)]
/// ShellCtx holds all the elements that will be rendered to the <head> of the page.
/// It can be modified by any component by accessing the context, but it's suggested to be used in
/// conjunction with the exported components <Dedup />, <Title />, <Html />, ....
pub struct ShellCtx {
    head_els: Rc<RefCell<Vec<Fragment>>>,
    deduped_head_els: Rc<RefCell<HashMap<String, Fragment>>>,
    html_attrs: Rc<RefCell<Attrs>>,
    body_attrs: Rc<RefCell<Attrs>>,
}

impl ShellCtx {
    pub fn new() -> Self {
        Self {
            head_els: Rc::new(RefCell::new(vec![])),
            deduped_head_els: Rc::new(RefCell::new(HashMap::new())),
            html_attrs: Rc::new(RefCell::new(Attrs::new())),
            body_attrs: Rc::new(RefCell::new(Attrs::new())),
        }
    }

    pub fn render(self, inner_body: String) -> String {
        let head = render_to_string(
            move || view! {{self.head_els.borrow().clone().collect_view()}{self.deduped_head_els.borrow().values().collect_view()}},
        );

        format!(
            "<!DOCTYPE html><html {}><head>{}</head><body {}>{}</body></html>",
            self.html_attrs.borrow().render(),
            head,
            self.body_attrs.borrow().render(),
            inner_body.trim(),
        )
    }
}

/// Attrs is a list of attributes.
pub struct Attrs {
    pub attrs: Vec<(String, String)>,
}

impl Attrs {
    pub fn new() -> Self {
        Self { attrs: vec![] }
    }

    pub fn render(&self) -> String {
        self.attrs
            .iter()
            .map(|(k, v)| format!("{k}=\"{v}\""))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn append(&mut self, other: &mut Self) {
        self.attrs.append(&mut other.attrs);
    }
}

impl From<Vec<(&str, &str)>> for Attrs {
    fn from(attrs: Vec<(&str, &str)>) -> Self {
        Self {
            attrs: attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl From<Vec<(String, String)>> for Attrs {
    fn from(attrs: Vec<(String, String)>) -> Self {
        Self { attrs }
    }
}
