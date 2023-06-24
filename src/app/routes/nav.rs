use leptos::{component, view, IntoView, Scope};
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="header">
            <nav class="inner">
                <A href="/">
                    <strong>"Shared Noise"</strong>
                </A>
                <A href="/about">
                    <strong>"About"</strong>
                </A>
                <A href="/resume">
                    <strong>"Resume"</strong>
                </A>                
        
                <A href="/blog">
                    <strong>"Blog"</strong>
                </A>        
                <a class="github" href="http://github.com/gbj/leptos" target="_blank" rel="noreferrer">
                    <span class="em-hammer_and_wrench"></span>
                    " with Leptos"
                </a>
            </nav>
        </header>
    }
}
