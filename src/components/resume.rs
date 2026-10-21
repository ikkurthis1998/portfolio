use super::ui::{Chips, DataError, Glyph, Icon, Loading, TextLink, EMAIL};
use crate::data::{fetch_portfolio_data, Experience, Project};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Resume() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    view! {
        <Title text="Resume — Sreemannarayana Ikkurthi"/>
        <div class="wrap page resume-page">
            // script.js drives the PDF states: Preparing (spinner), downloaded, failed, unavailable.
            <div class="pdf-bar">
                <button id="download-resume-btn" class="btn btn-primary" aria-describedby="pdf-status">
                    <Icon glyph=Glyph::Loader size=20 class="spinner"/><span class="btn-label">"Download PDF"</span>
                </button>
                <p id="pdf-status" class="t-label pdf-status" role="status" aria-live="polite"></p>
            </div>
            <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                Ok(d) => view! {
                    <article id="resume-content" class="card resume-doc">
                        <header class="resume-identity">
                            <h1 class="t-display">"Sreemannarayana Ikkurthi"</h1>
                            <p class="t-prose">"Software Architect"</p>
                            <div class="resume-contact">
                                <a href=format!("mailto:{EMAIL}")><Icon glyph=Glyph::Mail/>{EMAIL}</a>
                                <a href="https://isree.dev"><Icon glyph=Glyph::Globe/>"isree.dev"</a>
                            </div>
                        </header>
                        <section class="resume-section"><h2 class="eyebrow">"Summary"</h2><p class="t-prose">{d.summary}</p></section>
                        <section class="resume-section"><h2 class="eyebrow">"Technical Skills"</h2><Chips items=d.skills.into_iter().map(|s| s.name).collect()/></section>
                        <section class="resume-list"><h2 class="eyebrow">"Experience"</h2>{d.experiences.into_iter().map(|e| view! { <ResumeExperienceItem experience=e/> }).collect_view()}</section>
                        <section class="resume-list"><h2 class="eyebrow">"Projects"</h2>{d.projects.into_iter().take(4).map(|p| view! { <ResumeProjectItem project=p/> }).collect_view()}</section>
                    </article>
                }.into_any(),
                Err(_) => view! { <DataError/> }.into_any()
            })}</Suspense>
        </div>
    }
}

#[component]
fn ResumeExperienceItem(experience: Experience) -> impl IntoView {
    view! {
        <div class="resume-entry">
            <div class="role">
                <p class="t-caption c-3">{experience.duration}</p>
                <h3 class="t-headline">{experience.title}</h3>
                <p class="t-caption c-2">{experience.company}</p>
            </div>
            <div class="detail"><p class="t-prose">{experience.description}</p><Chips items=experience.technologies/></div>
        </div>
    }
}

#[component]
fn ResumeProjectItem(project: Project) -> impl IntoView {
    view! {
        <article class="resume-project">
            <div class="resume-project-head">
                <h3 class="resume-project-name"><span class="t-headline">{project.name.clone()}</span><span class="t-caption c-2">{project.language}</span></h3>
                <TextLink href=project.url label="View project" external=true/>
            </div>
            <p class="t-prose">{project.description}</p>
        </article>
    }
}
