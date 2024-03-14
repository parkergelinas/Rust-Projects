use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rust-fullstack.css"/>
        <Title text="Welcome to Leptos"/>
        <h1>"Hello World"</h1>
    }
}
