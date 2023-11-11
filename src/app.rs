use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod routes;
use routes::{home::*, resume::*};

// use crate::components::footer::*;
use crate::components::header::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Shared Noise - Maximum Volume Yeilds Maximum Results"/>

        // TODO add an error page
        <Router>
            <Header/>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/resume" view=Resume/>
                // <Route path="/posts" view=Posts/>
                // <Route path="/posts/:id" view=Posts/>
                </Routes>
            </main>
        // <Footer/>
        </Router>
    }
}
