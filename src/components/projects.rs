use super::ui::{BackLink, Chips, Contact, DataError, Loading, TextLink, ThemedImage, THELIVI_IOS_URL};
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

/// v2 project art (light and dark), exported from the design; other projects fall back
/// to their database image.
fn themed_art(project: &Project) -> Option<(&'static str, &'static str)> {
    match project_slug(project).as_str() {
        "intelligence" => Some(("/assets/v2/project-thelivi-light.webp", "/assets/v2/project-thelivi-dark.webp")),
        "airfoil-analysis" => Some(("/assets/v2/project-airfoil-light.webp", "/assets/v2/project-airfoil-dark.webp")),
        "genetic-algorithm-optimization" => Some(("/assets/v2/project-ga-light.webp", "/assets/v2/project-ga-dark.webp")),
        _ => None,
    }
}

#[component]
pub fn ProjectImage(project: Project, #[prop(default = true)] lazy: bool) -> impl IntoView {
    let alt = format!("{} — project preview", project.name);
    match (themed_art(&project), project.image.filter(|s| !s.is_empty())) {
        (Some((light, dark)), _) => view! { <ThemedImage light=light dark=dark alt=alt lazy=lazy/> }.into_any(),
        (None, Some(src)) => view! { <img src=src alt=alt loading="lazy"/> }.into_any(),
        (None, None) => ().into_any(),
    }
}

fn is_thelivi(project: &Project) -> bool {
    project_slug(project) == "intelligence"
}

#[component]
fn MobileAppLinks() -> impl IntoView {
    view! {
        <TextLink href=THELIVI_IOS_URL.into() label="iPhone app" external=true aria_label="Get Thelivi for iPhone on the App Store"/>
        <span class="t-caption c-3">"Android coming soon"</span>
    }
}

#[component]
pub fn WorkCard(project: Project, index: usize) -> impl IntoView {
    let href = format!("/projects/{}", project_slug(&project));
    view! {
        <article class="work-card" class:image-left=index % 2 == 1>
            <div class="work-copy">
                <div class="work-head"><p class="eyebrow">{category(&project)}</p>
                    <h2 class="t-display"><a href=href.clone()>{project.name.clone()}</a></h2>
                </div>
                <p class="t-prose">{project_summary(&project)}</p>
                <div class="work-links">
                    <TextLink href=href.clone() label="Explore project"/>
                    {is_thelivi(&project).then(|| view! { <MobileAppLinks/> })}
                </div>
            </div>
            <a class="work-image" href=href aria-label=format!("Explore {}", project.name) tabindex="-1"><ProjectImage project=project.clone()/></a>
        </article>
    }
}

const FILTERS: [&str; 4] = ["All", "Software", "Aerospace", "Algorithms"];

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    let (filter, set_filter) = signal("All");
    view! {
        <Title text="Selected work — Sreemannarayana Ikkurthi"/>
        <div class="wrap page">
            <header class="page-heading">
                <div class="heading-text">
                    <div class="heading-main"><p class="eyebrow">"Projects"</p><h1 class="t-display-web">"Selected work."</h1></div>
                    <p class="t-prose measure">"Working software and engineering experiments, from AI research tools to aerodynamic analysis."</p>
                </div>
                <div class="filters" role="group" aria-label="Filter projects">
                    {FILTERS.into_iter().map(|label| view! {
                        <button class="filter-chip" aria-pressed=move || (filter.get() == label).to_string() on:click=move |_| set_filter.set(label)>{label}</button>
                    }).collect_view()}
                </div>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                Ok(d) => {
                    let projects = ordered_projects(d.projects).into_iter().filter(|p| filter.get() == "All" || category(p) == filter.get()).collect::<Vec<_>>();
                    if projects.is_empty() { view! { <p class="t-prose empty-state">"No projects in this category yet."</p> }.into_any() }
                    else { view! { <div class="work-list">{projects.into_iter().enumerate().map(|(i, p)| view! { <WorkCard project=p index=i/> }).collect_view()}</div> }.into_any() }
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
        <Suspense fallback=|| view! { <div class="wrap page"><Loading/></div> }>{move || data.get().map(|result| match result {
            Ok(d) => {
                let slug = params.read().get("slug").unwrap_or_default();
                match d.projects.into_iter().find(|p| project_slug(p) == slug) {
                    Some(p) => view! { <ProjectDetail project=p/> }.into_any(),
                    None => view! { <ProjectNotFound/> }.into_any(),
                }
            },
            Err(_) => view! { <div class="wrap page"><DataError/></div> }.into_any()
        })}</Suspense>
    }
}

#[component]
fn ProjectDetail(project: Project) -> impl IntoView {
    let p = project;
    let title = format!("{} — Sreemannarayana Ikkurthi", p.name);
    let link_label = if p.url.contains("github.com") { "View source" } else { "Visit project" };
    view! {
        <Title text=title/>
        <div class="wrap page">
            <header class="detail-heading">
                <BackLink href="/projects" label=format!("Work / {}", category(&p))/>
                <h1 class="t-display-web">{p.name.clone()}</h1>
                <div class="detail-actions">
                    <a class="btn btn-primary" href=p.url.clone() target="_blank" rel="noopener noreferrer">{link_label}</a>
                    {is_thelivi(&p).then(|| view! { <MobileAppLinks/> })}
                    <span class="t-caption c-2">{p.language.clone()}</span>
                </div>
            </header>
            <div class="detail-image"><ProjectImage project=p.clone() lazy=false/></div>
            <section class="split" aria-labelledby="overview-title">
                <div class="lead"><p class="eyebrow">"Overview"</p><h2 id="overview-title" class="t-display">"What it does."</h2></div>
                <div class="prose">
                    <p class="t-prose">{p.description.clone()}</p>
                    {(!p.topics.is_empty()).then(|| view! { <h3 class="t-headline">"Tools & topics"</h3><Chips items=p.topics.clone()/> })}
                </div>
            </section>
            <div><BackLink href="/projects" label="All projects".into()/></div>
            <Contact/>
        </div>
    }
}

#[component]
fn ProjectNotFound() -> impl IntoView {
    view! {
        <Title text="Project not found — isree.dev"/>
        <div class="wrap page">
            <header class="page-heading">
                <div class="heading-main"><p class="eyebrow">"Projects"</p><h1 class="t-display-web">"Project not found."</h1></div>
                <div class="actions"><TextLink href="/projects".into() label="Explore all work"/></div>
            </header>
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
    #[test]
    fn every_featured_project_has_light_and_dark_art() {
        for name in ["Thelivi", "Airfoil Analysis", "Genetic Algorithm Optimization"] {
            let (light, dark) = themed_art(&project(name)).expect(name);
            assert!(light.ends_with("-light.webp") && dark.ends_with("-dark.webp"));
        }
        assert!(themed_art(&project("Other")).is_none());
    }
}
