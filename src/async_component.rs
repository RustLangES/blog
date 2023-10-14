use futures::Future;
use leptos::{
    component, create_resource, IntoView, Resource, Serializable, SerializationError, SignalGet,
    Suspense, SuspenseProps, View,
};

#[component]
/// Async wraps an async function that returns an IntoView into a Suspense.
///
/// This is useful when used together with async ssr that will wait on the provided async function
/// to render the final view.
pub fn Async<V, F, Fut>(view: F) -> impl IntoView
where
    V: IntoView + 'static,
    Fut: Future<Output = V> + 'static,
    F: Fn() -> Fut + 'static,
{
    // create an async resource that will return the view. It is wrapped in a Wrapper so that it
    // implements the Serializable trait. This works because it will only be rendered on the server
    // and never serialized.
    let once: Resource<(), Wrapper<View>> =
        create_resource(|| (), move |()| Wrapper::wrap_view(view()));

    Suspense(SuspenseProps {
        fallback: move || (),
        children: Box::new(move || once.get()),
    })
}

#[derive(Debug, Clone)]
/// Wrapper makes something implement the Serializable trait, any tries to serialize or deserialize
/// this will panic.
struct Wrapper<T>(T);

impl<T: IntoView> Wrapper<T> {
    pub async fn wrap_view<Fut: Future<Output = T>>(f: Fut) -> Wrapper<View> {
        Wrapper(f.await.into_view())
    }
}

impl<T> Serializable for Wrapper<T> {
    fn ser(&self) -> Result<String, SerializationError> {
        panic!("this should never be called");
    }

    fn de(_bytes: &str) -> Result<Self, SerializationError> {
        panic!("this should never be called");
    }
}

impl<T> IntoView for Wrapper<T>
where
    T: IntoView,
{
    fn into_view(self) -> View {
        self.0.into_view()
    }
}
