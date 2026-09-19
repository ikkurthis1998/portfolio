use super::editorial::{Contact, DataError, Loading};
use crate::data::{Project, fetch_portfolio_data};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

pub fn project_slug(project: &Project) -> String {
    // Keep existing portfolio URLs stable after the product rename.
    if project.name.eq_ignore_ascii_case("Thelivi") {
        return "intelligence".into();
    }
    project
        .name
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn ordered_projects(mut projects: Vec<Project>) -> Vec<Project> {
    projects.sort_by_key(|p| match project_slug(p).as_str() {
        "intelligence" => 0,
        "airfoil-analysis" => 1,
        "genetic-algorithm-optimization" => 2,
        _ => 3,
    });
    projects
}

fn project_summary(project: &Project) -> String {
    match project_slug(project).as_str() {
        "intelligence" => "An AI agent platform for research and ongoing work, with shared project knowledge, memory, and scheduled tasks.".into(),
        "airfoil-analysis" => "Exploring aerodynamic performance through computation. A Python tool for airfoil analysis with XFOIL.".into(),
        "genetic-algorithm-optimization" => "Finding better solutions through evolutionary search, with contour-plot visualization.".into(),
        _ => project.description.clone(),
    }
}

fn category(project: &Project) -> &'static str {
    let text = format!("{} {}", project.name, project.topics.join(" ")).to_lowercase();
    if text.contains("airfoil") || text.contains("aerospace") {
        "Aerospace"
    } else if text.contains("algorithm") || text.contains("optimization") {
        "Algorithms"
    } else {
        "Software"
    }
}

#[component]
pub fn ProjectImage(project: Project) -> impl IntoView {
    let name = project.name.clone();
    match project.image.filter(|s| !s.is_empty()) {
        Some(src) => view! { <img src=src alt=format!("{} — project preview", name) loading="lazy" width="1200" height="630"/> }.into_any(),
        None => view! { <div class="project-image-fallback"><span class="mono">"PROJECT / EXPLORATION"</span><strong>{name}</strong></div> }.into_any(),
    }
}

#[component]
pub fn ProjectRow(project: Project, index: usize) -> impl IntoView {
    let href = format!("/projects/{}", project_slug(&project));
    let has_mobile_app = project_slug(&project) == "intelligence";
    let category = category(&project);
    view! {
        <article class="project-row" class:reverse=index % 2 == 1>
            <div class="project-copy"><p class="eyebrow">{category}</p>
                <h2><a href=href.clone()>{project.name.clone()}</a></h2>
                <p class="project-description">{project_summary(&project)}</p>
                <a class="text-link" href=href.clone()>"Explore project ↗"</a>
                {has_mobile_app.then(|| view! { <div class="project-app-links"><MobileAppLinks/></div> })}
            </div>
            <a class="project-image" href=href aria-label=format!("Explore {}", project.name)><ProjectImage project=project.clone()/></a>
        </article>
    }
}

#[component]
fn MobileAppLinks() -> impl IntoView {
    view! {
        <a class="text-link" href="https://apps.apple.com/app/id6790257232" target="_blank" rel="noopener noreferrer" aria-label="Get Thelivi for iPhone on the App Store">"iPhone app ↗"</a>
        <span class="mono" title="Currently in closed testing">"Android — coming soon"</span>
    }
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    let (filter, set_filter) = signal("All");
    view! {
        <Title text="Selected work — Sreemannarayana Ikkurthi"/>
        <div class="page-wrap project-list">
            <header class="page-heading"><p class="eyebrow">"Projects"</p><h1>"Selected "<span>"work."</span></h1><p>"Working software and engineering experiments, from AI research tools to aerodynamic analysis."</p></header>
            <div class="project-filters" role="group" aria-label="Filter projects">
                {["All", "Software", "Aerospace", "Algorithms"].into_iter().map(|label| view! {
                    <button class:active=move || filter.get() == label aria-pressed=move || (filter.get() == label).to_string() on:click=move |_| set_filter.set(label)>{label}</button>
                }).collect_view()}
            </div>
            <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                Ok(d) => {
                    let projects = ordered_projects(d.projects).into_iter().filter(|p| filter.get() == "All" || category(p) == filter.get()).collect::<Vec<_>>();
                    if projects.is_empty() { view! { <p class="empty-state">"No projects in this category yet."</p> }.into_any() }
                    else { projects.into_iter().enumerate().map(|(i,p)| view! { <ProjectRow project=p index=i/> }).collect_view().into_any() }
                },
                Err(_) => view! { <DataError/> }.into_any()
            })}</Suspense>
            <Contact/>
        </div>
    }
}

#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    view! {
        <div class="page-wrap">
            <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                Ok(d) => {
                    let slug = params.read().get("slug").unwrap_or_default();
                    let project = d.projects.into_iter().find(|p| project_slug(p) == slug);
                    match project {
                        Some(p) => {
                            let title = format!("{} — Sreemannarayana Ikkurthi", p.name);
                            let link_label = if p.url.contains("github.com") { "View source ↗" } else { "Visit project ↗" };
                            view! {
                                <Title text=title/>
                                <header class="page-heading detail-heading"><a class="eyebrow" href="/projects">"← Work / "{category(&p)}</a><h1>{p.name.clone()}</h1>
                                    <div class="actions"><a class="button" href=p.url.clone() target="_blank" rel="noopener noreferrer">{link_label}</a>{(project_slug(&p) == "intelligence").then(|| view! { <MobileAppLinks/> })}<span class="mono">{p.language.clone()}</span></div>
                                </header>
                                <div class="detail-image"><ProjectImage project=p.clone()/></div>
                                <section class="narrative-row"><div><p class="eyebrow">"Overview"</p><h2>"What it does."</h2></div><div class="prose"><p>{p.description}</p>
                                    <h3>"Tools & topics"</h3><ul class="topic-list">{p.topics.into_iter().map(|t| view! { <li>{t}</li> }).collect_view()}</ul>
                                </div></section>
                                <a class="text-link" href="/projects">"← All projects"</a>
                                <Contact/>
                            }.into_any()
                        },
                        None => view! { <Title text="Project not found — isree.dev"/><div class="page-heading"><h1>"Project not found."</h1><a class="text-link" href="/projects">"Explore all work ↗"</a></div> }.into_any()
                    }
                },
                Err(_) => view! { <DataError/> }.into_any()
            })}</Suspense>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn project(name: &str) -> Project {
        Project {
            name: name.into(),
            description: String::new(),
            language: String::new(),
            stars: 0,
            url: String::new(),
            topics: vec![],
            image: None,
        }
    }
    #[test]
    fn slugs_are_readable_and_stable() {
        assert_eq!(
            project_slug(&project("Genetic Algorithm Optimization")),
            "genetic-algorithm-optimization"
        );
        assert_eq!(project_slug(&project("Systems / AI")), "systems-ai");
    }
    #[test]
    fn featured_order_preserves_remaining_database_order() {
        let ordered = ordered_projects(vec![
            project("Other A"),
            project("Airfoil Analysis"),
            project("Other B"),
            project("Intelligence"),
        ]);
        assert_eq!(
            ordered.into_iter().map(|p| p.name).collect::<Vec<_>>(),
            ["Intelligence", "Airfoil Analysis", "Other A", "Other B"]
        );
    }
}
