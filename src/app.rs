use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod routes;
use routes::{nav::*, resume::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Shared Noise - Maximum Volume Yeilds Maximum Results"/>

        // content for this welcome page
        <Router>
            <Nav />
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="resume" view=|cx| view!{ cx, <Resume/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: leptos::Scope) -> impl IntoView {

    view! { cx,
        <div class="parallax">
        </div>                
    }
}
