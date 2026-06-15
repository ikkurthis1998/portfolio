use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::{Route, Router, Routes}, path};
use crate::components::*;
use crate::pages::analytics::Analytics;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <html>
        <head>
            <MetaTags/>
            <Link rel="icon" type_="image/png" href="/assets/logo.png"/>
            <Title text="Sreemannarayana Ikkurthi"/>
            <Meta name="description" content="Portfolio of Sreemannarayana Ikkurthi - Software Engineer"/>
            <Link rel="stylesheet" href="/pkg/portfolio.css"/>
            <script src="https://cdn.tailwindcss.com"></script>
            <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            // External scripts loaded from CDN
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
                            <Route path=path!("/analytics") view=Analytics/>
                        </Routes>
                    </main>
                </div>
            </Router>
        </body>
        </html>
    }
}
