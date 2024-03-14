use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::blog_previews::BlogPreviews;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav>"This is a navbar"</nav>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rust-fullstack.css"/>
        <Title text="Welcome to Leptos"/>

        <Navbar />

        <Router>
            <main>
                <Routes>
                    <Route path="" view=BlogPreviews />
                    <Route path="/edit/:post_id?" view=EditPost/>
                    <Route path="/view/:post_id?" view=ViewPost/>
                </Routes>
            </main>
        </Router>
    }
}
