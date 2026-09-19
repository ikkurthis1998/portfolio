use super::editorial::{DataError, Loading};
use crate::data::{fetch_portfolio_data, Experience, Project};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Resume() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    view! {
        <Title text="Resume — Sreemannarayana Ikkurthi"/>
        <div class="classic-resume resume-page">
            <div class="resume-download"><button id="download-resume-btn" class="button" aria-describedby="pdf-status">"Download PDF ↓"</button></div>
            <p id="pdf-status" class="mono" role="status" aria-live="polite"></p>
            <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                Ok(d) => view! {
                    <div id="resume-content">
                        <header class="resume-identity"><img class="resume-portrait" src="/assets/IMG_4872.JPG" alt="Sreemannarayana Ikkurthi"/><h1>"Sreemannarayana Ikkurthi"</h1><p>"Software Architect"</p><div class="resume-contact"><a href="mailto:ikkurthis1998@gmail.com"><ContactIcon kind=0/>"ikkurthis1998@gmail.com"</a><a href="https://isree.dev"><ContactIcon kind=1/>"isree.dev"</a><a href="https://github.com/ikkurthis1998"><ContactIcon kind=2/>"github.com/ikkurthis1998"</a><a href="https://linkedin.com/in/ikkurthis1998"><ContactIcon kind=3/>"linkedin.com/in/ikkurthis1998"</a></div></header>
                        <section class="resume-summary"><h3 class="eyebrow">"Summary"</h3><p>{d.summary}</p></section>
                        <section class="resume-skills"><h3 class="eyebrow">"Technical Skills"</h3><ul class="topic-list">{d.skills.into_iter().map(|s| view! { <li>{s.name}</li> }).collect_view()}</ul></section>
                        <section class="resume-experience"><h3 class="eyebrow">"Experience"</h3>{d.experiences.into_iter().map(|e| view! { <ResumeExperienceItem experience=e/> }).collect_view()}</section>
                        <section class="resume-projects"><h3 class="eyebrow">"Projects"</h3>{d.projects.into_iter().take(4).map(|p| view! { <ResumeProjectItem project=p/> }).collect_view()}</section>
                    </div>
                }.into_any(),
                Err(_) => view! { <DataError/> }.into_any()
            })}</Suspense>
        </div>
    }
}

// Resume-specific rows: website card styling must not change the document.
#[component]
fn ContactIcon(kind: u8) -> impl IntoView {
    let path = match kind {
        0 => "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
        1 => "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9",
        2 => "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4",
        _ => "M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z",
    };
    view! { <svg aria-hidden="true" viewBox="0 0 24 24" fill=if kind == 3 { "currentColor" } else { "none" } stroke=if kind == 3 { "none" } else { "currentColor" } stroke-width="2"><path d=path stroke-linecap="round" stroke-linejoin="round"/></svg> }
}

#[component]
fn ResumeExperienceItem(experience: Experience) -> impl IntoView {
    view! {
        <div class="resume-entry experience-row">
            <span class="mono">{experience.duration}</span>
            <div class="resume-role"><h3>{experience.title}</h3><div class="company">{experience.company}</div></div>
            <div class="prose"><p>{experience.description}</p>
                <ul class="topic-list">{experience.technologies.into_iter().map(|t| view! { <li>{t}</li> }).collect_view()}</ul>
            </div>
        </div>
    }
}

#[component]
fn ResumeProjectItem(project: Project) -> impl IntoView {
    view! {
        <article class="resume-project-entry">
            <div class="resume-project-heading"><h4>{project.name}<span class="resume-language">" - "{project.language}</span></h4>
                <a href=project.url target="_blank" rel="noopener noreferrer">"View project"</a>
            </div>
            <p>{project.description}</p>
        </article>
    }
}
