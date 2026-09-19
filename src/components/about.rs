use super::editorial::{SystemsArtwork, Approach, Contact, DataError, Loading};
use crate::data::{Experience, fetch_portfolio_data};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn AboutPage() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    view! {
        <Title text="About — Sreemannarayana Ikkurthi"/>
        <div class="page-wrap">
            <header class="page-heading about-heading"><div><p class="eyebrow">"About / Sreemannarayana Ikkurthi"</p><h1>"Understand the problem."<br/><span>"Engineer the solution."</span></h1><p>"Aerospace foundations. Experience in EV infrastructure, software architecture, and AI products."</p></div><SystemsArtwork builder=true/></header>
            <section class="narrative-row"><div><p class="eyebrow">"From aerospace to software"</p><h2>"Different disciplines."<br/><span>"The same method."</span></h2></div><div class="prose"><p>"My engineering work began in aerospace, with airfoil analysis and rocket motor test-pad design. I later moved into software for EV charging networks, then into AI products."</p><p>"Across these fields, my approach is consistent: understand the constraints, test the assumptions, and build a system that works in practice."</p></div></section>
            <Approach/>
            <section class="experience-section"><div class="section-bar"><h2 class="eyebrow">"Experience"</h2><a class="mono" href="/resume">"View resume ↗"</a></div>
                <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                    Ok(d) => d.experiences.into_iter().map(|e| view! { <ExperienceCard experience=e/> }).collect_view().into_any(),
                    Err(_) => view! { <DataError/> }.into_any()
                })}</Suspense>
            </section><Contact/>
        </div>
    }
}

#[component]
pub fn ExperienceCard(experience: Experience) -> impl IntoView {
    view! { <article class="experience-row"><p class="mono">{experience.duration}</p><div><h3>{experience.title}</h3><p class="company">{experience.company}</p></div><div class="prose"><p>{experience.description}</p><ul class="topic-list">{experience.technologies.into_iter().map(|t| view! { <li>{t}</li> }).collect_view()}</ul></div></article> }
}
