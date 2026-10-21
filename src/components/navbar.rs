use super::ui::{Glyph, Icon, Lockup};
use leptos::prelude::*;
use leptos_router::hooks::use_location;

const LINKS: [(&str, &str); 4] = [("/projects", "Work"), ("/about", "About"), ("/resume", "Resume"), ("/hire", "Hire me")];

fn is_current(path: &str, href: &str) -> bool {
    path == href || path.starts_with(&format!("{href}/"))
}

#[component]
pub fn NavBar() -> impl IntoView {
    let (open, set_open) = signal(false);
    let path = use_location().pathname;
    // Any route change (including the menu's own links) closes the mobile menu.
    Effect::new(move |_| {
        path.track();
        set_open.set(false);
    });
    let current = move |href: &'static str| move || is_current(&path.get(), href).then_some("page");
    view! {
        <header class="site-header">
            <div class="wrap nav-bar">
                <Lockup/>
                <nav class="nav-links" aria-label="Main navigation">
                    {LINKS.iter().map(|&(href, label)| view! { <a class="nav-link" href=href aria-current=current(href)>{label}</a> }).collect_view()}
                    <a class="btn btn-primary btn-sm" href="/#contact">"Let’s talk"</a>
                </nav>
                <button class="menu-button" aria-label=move || if open.get() { "Close navigation" } else { "Open navigation" }
                    aria-expanded=move || open.get().to_string() aria-controls="mobile-navigation" on:click=move |_| set_open.update(|v| *v = !*v)>
                    {move || if open.get() { view! { <Icon glyph=Glyph::Close size=20/> }.into_any() } else { view! { <Icon glyph=Glyph::Menu size=20/> }.into_any() }}
                </button>
            </div>
            <nav id="mobile-navigation" class="mobile-menu" class:is-open=move || open.get() aria-label="Main navigation" on:click=move |_| set_open.set(false)>
                <div class="menu-rows">
                    {LINKS.iter().map(|&(href, label)| view! { <a href=href aria-current=current(href)>{label}</a> }).collect_view()}
                </div>
                <a class="btn btn-primary" href="/#contact">"Let’s talk"</a>
            </nav>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="wrap footer-bar">
                <Lockup/>
                <p class="t-caption footer-tagline">"From first principles to working products."</p>
                <a class="text-link" href="#main-content"><span class="label">"Back to top"</span><Icon glyph=Glyph::ArrowUp/></a>
            </div>
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use super::is_current;
    #[test]
    fn work_stays_current_on_project_pages() {
        assert!(is_current("/projects", "/projects"));
        assert!(is_current("/projects/intelligence", "/projects"));
        assert!(!is_current("/projectsx", "/projects"));
        assert!(!is_current("/", "/about"));
    }
}
