use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::{Route, Router, Routes}, path};
use crate::components::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <html>
        <head>
            <MetaTags/>
            <Link rel="icon" type_="image/png" href="/assets/logo.png"/>
            <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
            <Title text="Sreemannarayana Ikkurthi - Portfolio"/>
            <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            <script src="https://cdn.tailwindcss.com"></script>
            <script src="/assets/html2pdf.bundle.min.js"></script>
            <script src="/assets/script.js"></script>
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
                            <Route path=path!("/resume") view=Resume/>
                        </Routes>
                    </main>
                </div>
            </Router>
        </body>
        </html>
    }
}
