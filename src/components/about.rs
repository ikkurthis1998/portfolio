use super::ui::{Chips, Contact, DataError, HowIWork, Loading, TextLink};
use crate::data::{Experience, fetch_portfolio_data};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn AboutPage() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    view! {
        <Title text="About — Sreemannarayana Ikkurthi"/>
        <div class="wrap page">
            <header class="page-heading">
                <div class="heading-text">
                    <div class="heading-main"><p class="eyebrow">"About / Sreemannarayana Ikkurthi"</p>
                        <h1 class="t-display-web">"Understand the problem."<br class="br-lg"/>" Engineer the solution."</h1></div>
                    <p class="t-prose measure">"Aerospace foundations. Experience in EV infrastructure, software architecture, and AI products."</p>
                </div>
            </header>
            <section class="split" aria-labelledby="method-title">
                <div class="lead"><p class="eyebrow">"From aerospace to software"</p><h2 id="method-title" class="t-display">"Different disciplines."<br/>"The same method."</h2></div>
                <div class="prose">
                    <p class="t-prose">"My engineering work began in aerospace, with airfoil analysis and rocket motor test-pad design. I later moved into software for EV charging networks, then into AI products."</p>
                    <p class="t-prose">"Across these fields, my approach is consistent: understand the constraints, test the assumptions, and build a system that works in practice."</p>
                </div>
            </section>
            <HowIWork/>
            <section class="exp-list" aria-labelledby="experience-title">
                <div class="section-bar"><h2 id="experience-title" class="eyebrow">"Experience"</h2><TextLink href="/resume".into() label="View resume"/></div>
                <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                    Ok(d) => d.experiences.into_iter().map(|e| view! { <ExperienceCard experience=e/> }).collect_view().into_any(),
                    Err(_) => view! { <DataError/> }.into_any()
                })}</Suspense>
            </section>
            <Contact/>
        </div>
    }
}

#[component]
pub fn ExperienceCard(experience: Experience) -> impl IntoView {
    view! {
        <article class="card exp-card">
            <div class="role">
                <p class="t-caption c-3">{experience.duration}</p>
                <h3 class="t-headline">{experience.title}</h3>
                <p class="t-caption c-2">{experience.company}</p>
            </div>
            <div class="detail"><p class="t-prose">{experience.description}</p><Chips items=experience.technologies/></div>
        </article>
    }
}
