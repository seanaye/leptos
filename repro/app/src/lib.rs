use leptos::*;

mod components;

pub use components::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Article />
    }
}
