use leptos::prelude::*;

#[component]
pub fn SystemsArtwork(#[prop(default = false)] builder: bool) -> impl IntoView {
    let source = if builder { "/assets/builders-object.png" } else { "/assets/living-systems.png" };
    let prefix = if builder { "builder" } else { "landscape" };
    view! {
        <figure class=if builder { "systems-artwork artwork-builder" } else { "systems-artwork artwork-flow" }>
            // The original artwork is retained, not approximated with new geometry.
            // About keeps the complete silhouette; its label is excluded locally in CSS.
            <svg viewBox=if builder { "0 0 1536 1024" } else { "0 100 1536 850" } role="img" aria-label=if builder { "Detailed exploded engineering sculpture with translucent shells and a blue core, gently moving in depth" } else { "Fine flowing lines connecting a computational lattice to a sculptural landscape" }>
                <defs>
                    <linearGradient id=format!("{prefix}-left-fade")><stop offset="0.25" stop-color="white"/><stop offset="0.44" stop-color="black"/></linearGradient>
                    <linearGradient id=format!("{prefix}-right-fade")><stop offset="0.57" stop-color="black"/><stop offset="0.76" stop-color="white"/></linearGradient>
                    <linearGradient id=format!("{prefix}-middle-fade")><stop offset="0.25" stop-color="black"/><stop offset="0.44" stop-color="white"/><stop offset="0.57" stop-color="white"/><stop offset="0.76" stop-color="black"/></linearGradient>
                    <mask id=format!("{prefix}-left") maskUnits="userSpaceOnUse" x="0" y="0" width="1536" height="1024"><rect width="1536" height="1024" fill=format!("url(#{prefix}-left-fade)")/></mask>
                    <mask id=format!("{prefix}-middle") maskUnits="userSpaceOnUse" x="0" y="0" width="1536" height="1024"><rect width="1536" height="1024" fill=format!("url(#{prefix}-middle-fade)")/></mask>
                    <mask id=format!("{prefix}-right") maskUnits="userSpaceOnUse" x="0" y="0" width="1536" height="1024"><rect width="1536" height="1024" fill=format!("url(#{prefix}-right-fade)")/></mask>
                </defs>
                <image class="art-original" href=source width="1536" height="1024"/>
                <g class="art-depth art-depth-front" mask=format!("url(#{prefix}-left)")><image href=source width="1536" height="1024"/></g>
                <g class="art-depth art-depth-middle" mask=format!("url(#{prefix}-middle)")><image href=source width="1536" height="1024"/></g>
                <g class="art-depth art-depth-back" mask=format!("url(#{prefix}-right)")><image href=source width="1536" height="1024"/></g>
            </svg>
        </figure>
    }
}
#[component]
pub fn Loading() -> impl IntoView {
    view! { <div class="loading-state" role="status">"Loading portfolio"<span class="loading-line"></span></div> }
}

#[component]
pub fn DataError() -> impl IntoView {
    view! { <div class="data-error" role="alert"><h2>"The content couldn’t load."</h2><p>"Please try again in a moment."</p><a class="text-link" href="">"Reload page ↗"</a></div> }
}

#[component]
pub fn Approach() -> impl IntoView {
    view! {
        <section class="approach-section" aria-labelledby="approach-title">
            <p class="eyebrow section-label">"Approach"</p>
            <div class="approach-grid"><h2 id="approach-title">"Start with the problem."<br/><span>"Test what you build."</span></h2>
                <div><span class="mono">"01"</span><h3>"Define the problem"</h3><p>"Identify who needs the system, what it must do, and the constraints it has to work within."</p></div>
                <div><span class="mono">"02"</span><h3>"Build a working version"</h3><p>"Choose an architecture that fits the problem. Build the core workflow and test it end to end."</p></div>
                <div><span class="mono">"03"</span><h3>"Measure and improve"</h3><p>"Use tests, operational feedback, and real usage to improve reliability and remove unnecessary complexity."</p></div>
            </div>
        </section>
    }
}

#[component]
pub fn Contact() -> impl IntoView {
    view! { <section id="contact" class="contact-section"><p class="eyebrow">"Let’s connect"</p><div class="contact-grid"><div><h2>"Have a problem to solve?"</h2><p>"Let’s discuss the requirements and what it would take to build."</p><a class="text-link" href="mailto:ikkurthis1998@gmail.com">"Get in touch ↗"</a></div><div class="social-links"><span class="mono">"Find me on"</span><a href="https://github.com/ikkurthis1998" target="_blank" rel="noopener noreferrer">"GitHub ↗"</a><a href="https://linkedin.com/in/ikkurthis1998" target="_blank" rel="noopener noreferrer">"LinkedIn ↗"</a></div></div></section> }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! { <footer class="site-footer page-wrap"><a class="brand" href="/"><img src="/assets/logo.png" alt="Sree’s logo"/><span>"isree.dev"</span></a><p>"From first principles to working products."</p><a href="#main-content" class="mono">"Back to top ↑"</a></footer> }
}
