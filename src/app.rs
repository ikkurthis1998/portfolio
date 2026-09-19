use crate::components::*;
use crate::pages::analytics::Analytics;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Link rel="icon" type_="image/png" href="/assets/logo.png"/>
        <Title text="Sreemannarayana Ikkurthi — Software & Systems"/>
        <Meta name="description" content="From first principles to working products. Sreemannarayana Ikkurthi builds software systems across EV infrastructure, AI products, and computational engineering."/>
        <Link rel="stylesheet" href="/assets/editorial.css?v=project-artwork-light-1"/>
        <Script src="/assets/html2pdf.bundle.min.js"/>
        <Script src="/assets/script.js?v=resume-ui-export-5"/>
        <Router>
            <a class="skip-link" href="#main-content">"Skip to content"</a>
            <div id="root">
                <NavBar/>
                <main id="main-content">
                    <Routes fallback=|| view! { <div class="page-wrap page-heading"><h1>"Page not found."</h1><a class="text-link" href="/">"Back home ↗"</a></div> }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/projects") view=ProjectsPage/>
                        <Route path=path!("/projects/:slug") view=ProjectDetailPage/>
                        <Route path=path!("/about") view=AboutPage/>
                        <Route path=path!("/resume") view=Resume/>
                        <Route path=path!("/analytics") view=Analytics/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
