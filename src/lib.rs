use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub mod components;
pub mod data;

use components::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <html>
        <head>
            <MetaTags/>
            <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
            <Title text="Sreemannarayana Ikkurthi - Portfolio"/>
            <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            <meta name="description" content="Sreemannarayana Ikkurthi - Full Stack Developer & Aerospace Engineer"/>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body>
            <Router>
                <div id="root">
                    <NavBar/>
                    <main>
                        <Routes fallback=|| "Page not found.">
                            <Route path=path!("/") view=HomePage/>
                            <Route path=path!("/projects") view=ProjectsPage/>
                            <Route path=path!("/about") view=AboutPage/>
                            <Route path=path!("/contact") view=ContactPage/>
                        </Routes>
                    </main>
                </div>
            </Router>
        </body>
        </html>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    
    // Log that hydration is starting
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"🚀 Portfolio WASM hydrating...".into());
    
    leptos::mount::hydrate_body(App);
    
    // Log that hydration is complete
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"✅ Portfolio WASM hydration complete!".into());
}

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    
    // Log that CSR is starting
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"🚀 Portfolio CSR starting...".into());
    
    leptos::mount::mount_to_body(App);
    
    // Log that CSR is complete
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"✅ Portfolio CSR mount complete!".into());
}
