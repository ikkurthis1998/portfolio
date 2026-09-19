use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavBar() -> impl IntoView {
    let (open, set_open) = signal(false);
    view! {
        <header class="site-header page-wrap">
            <a class="brand" href="/" aria-label="Sree — Home"><img src="/assets/logo.png" alt=""/><span>"isree.dev"</span></a>
            <button class="menu-button" aria-label=move || if open.get() { "Close navigation" } else { "Open navigation" }
                aria-expanded=move || open.get().to_string() aria-controls="primary-navigation" on:click=move |_| set_open.update(|v| *v = !*v)>
                {move || if open.get() { "Close −" } else { "Menu +" }}
            </button>
            <nav id="primary-navigation" class="primary-nav" class:is-open=move || open.get() aria-label="Main navigation" on:click=move |_| set_open.set(false)>
                <A href="/projects">"Work"</A><A href="/about">"About"</A><A href="/resume">"Resume"</A>
                <a href="/#contact">"Let’s talk ↗"</a>
            </nav>
        </header>
    }
}
