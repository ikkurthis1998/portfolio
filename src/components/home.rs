use super::{
    editorial::{SystemsArtwork, Approach, Contact, DataError, Loading},
    projects::{ProjectRow, ordered_projects},
};
use crate::data::fetch_portfolio_data;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    view! {
        <div class="page-wrap">
            <section class="hero">
                <div class="hero-copy"><p class="eyebrow">"Software architect / Aerospace background"</p>
                    <h1>"From first principles"<br/><span>"to working products."</span></h1>
                    <p class="hero-intro">"I’m Sreemannarayana. I design and build software systems—from EV charging infrastructure to AI products."</p>
                    <div class="actions"><a class="button" href="#projects">"Explore my work ↗"</a><a class="text-link" href="/about">"About me"</a></div>
                </div><SystemsArtwork/>
            </section>
            <section id="projects" class="project-list" aria-labelledby="selected-work"><div class="section-bar"><h2 id="selected-work" class="eyebrow">"Work"</h2><a class="mono" href="/projects">"View all projects ↗"</a></div>
                <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                    Ok(d) => ordered_projects(d.projects).into_iter().take(3).enumerate().map(|(i,p)| view! { <ProjectRow project=p index=i/> }).collect_view().into_any(),
                    Err(_) => view! { <DataError/> }.into_any(),
                })}</Suspense>
            </section>
            <Approach/><Contact/>
        </div>
    }
}
