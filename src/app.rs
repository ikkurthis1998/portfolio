use crate::components::*;
use crate::components::ui::TextLink;
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
        <Link rel="icon" type_="image/png" sizes="32x32" href="/assets/icons/icon-32.png"/>
        <Link rel="icon" type_="image/png" sizes="48x48" href="/assets/icons/icon-48.png"/>
        <Link rel="icon" type_="image/svg+xml" href="/assets/favicon.svg?v=2"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/assets/icons/apple-touch-icon.png"/>
        <Title text="Sreemannarayana Ikkurthi — Software & Systems"/>
        <Meta name="description" content="From first principles to working products. Sreemannarayana Ikkurthi builds software systems across EV infrastructure, AI products, and computational engineering."/>
        <Link rel="preload" href="/assets/fonts/nunito-latin-wght-normal.woff2" as_="font" type_="font/woff2" crossorigin="anonymous"/>
        <Link rel="stylesheet" href="/assets/v2.css?v=3"/>
        // Resume: .classic-resume is the printable document, .resume-page its on-screen theme.
        <Link rel="stylesheet" href="/assets/resume.css?v=themed-2"/>
        <Script src="/assets/html2pdf.bundle.min.js"/>
        <Script src="/assets/script.js?v=resume-themed-2"/>
        <Router>
            <a class="skip-link" href="#main-content">"Skip to content"</a>
            <div id="root">
                <NavBar/>
                <main id="main-content">
                    <Routes fallback=NotFound>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/projects") view=ProjectsPage/>
                        <Route path=path!("/projects/:slug") view=ProjectDetailPage/>
                        <Route path=path!("/about") view=AboutPage/>
                        <Route path=path!("/hire") view=HirePage/>
                        <Route path=path!("/resume") view=Resume/>
                        <Route path=path!("/vihari/privacy") view=VihariPrivacyPage/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="wrap page">
            <header class="page-heading">
                <h1 class="t-display-web">"Page not found."</h1>
                <div class="actions"><TextLink href="/".into() label="Back home"/></div>
            </header>
        </div>
    }
}
