use leptos::{component, view, CollectView, IntoAttribute, IntoView};
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [
        ("About", "/about"),
        ("Posts", "/posts"),
        ("Resume", "/resume"),
    ];
    view! {
        <header class="header">
            <nav class="inner">
                {nav_items
                    .iter()
                    .map(|(name, href)| {
                        view! {
                            <a href=href.to_string()>
                                <strong>{name.to_string()}</strong>
                            </a>
                        }
                    })
                    .collect_view()}
                <a
                    class="github"
                    href="http://github.com/gbj/leptos"
                    target="_blank"
                    rel="noreferrer"
                >
                    <span class="em-hammer_and_wrench"></span>
                    " with Leptos"
                </a>
            </nav>
        </header>
    }
}
