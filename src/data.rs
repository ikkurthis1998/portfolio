use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub language: String,
    pub stars: i32,
    pub url: String,
    pub topics: Vec<String>,
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub duration: String,
    pub description: String,
    pub technologies: Vec<String>,
    #[serde(default)]
    pub logo: Option<String>,
    #[serde(default)]
    pub logo_zoom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub name: String,
    pub level: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfolioData {
    pub about: String,
    pub summary: String,
    pub projects: Vec<Project>,
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
}

// Curated from https://www.linkedin.com/in/ikkurthis1998/ on 2026-09-14.
// Shared by About, Resume and the local snapshot; no production database writes.
#[cfg(feature = "ssr")]
fn current_experiences() -> Vec<Experience> {
    serde_json::from_str(include_str!("../content/experience.json"))
        .expect("checked-in experience content must be valid")
}

// Reviewed against Thelivi's README and Projects/Memory guides on 2026-09-14.
// Apply to both preview and database content so every portfolio view stays consistent.
#[cfg(feature = "ssr")]
fn current_projects(mut projects: Vec<Project>) -> Vec<Project> {
    for project in &mut projects {
        if project.name.eq_ignore_ascii_case("Intelligence") || project.name.eq_ignore_ascii_case("Thelivi") {
            project.name = "Thelivi".into();
            project.url = "https://thelivi.isree.dev/".into();
            project.image = Some("/assets/project-thelivi-light.png".into());
            project.description = "Thelivi is an AI agent platform for research and ongoing work. It combines web browsing, document search, shared project knowledge, persistent memory, and scheduled tasks. Built with Go, TypeScript, Temporal, and SurrealDB, with an embeddable assistant for websites.".into();
        } else if project.name.eq_ignore_ascii_case("Airfoil Analysis") {
            project.image = Some("/assets/project-airfoil-light.png".into());
        } else if project.name.eq_ignore_ascii_case("Genetic Algorithm Optimization") {
            project.image = Some("/assets/project-optimization-light.png".into());
        }
    }
    projects
}

#[server(FetchPortfolioData, "/api")]
pub async fn fetch_portfolio_data() -> Result<PortfolioData, ServerFnError> {
    // An explicit development-only snapshot of already-public content for offline UI review.
    #[cfg(debug_assertions)]
    if let Ok(path) = std::env::var("PORTFOLIO_PREVIEW_DATA") {
        let json = std::fs::read_to_string(path).map_err(|_| ServerFnError::new("Preview data unavailable"))?;
        let mut data: PortfolioData = serde_json::from_str(&json).map_err(|_| ServerFnError::new("Invalid preview data"))?;
        data.experiences = current_experiences();
        data.projects = current_projects(data.projects);
        return Ok(data);
    }
    use crate::db::get_db;
    let db = get_db().await?;

    // One round-trip: profile + the three ordered lists. Ordering is applied
    // Rust-side from the migrated keys (display_order / start_date / seq) to
    // avoid SurrealDB NULLS-LAST ordering quirks on small result sets.
    let sql = "SELECT about, summary FROM profile LIMIT 1;\n\
               SELECT * FROM project;\n\
               SELECT * FROM experience;\n\
               SELECT * FROM skill;";
    let r = db
        .query(sql)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // profile
    let prof = r
        .first()
        .and_then(|v| v.as_array())
        .and_then(|a| a.first());
    let about = prof
        .and_then(|o| o.get("about"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let summary = prof
        .and_then(|o| o.get("summary"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| about.clone());

    // projects — order by display_order asc, then seq asc
    let mut projects: Vec<(i64, i64, Project)> = r
        .get(1)
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|o| {
                    let p: Project = from_row(o, "project")?;
                    Some((int_field(o, "display_order"), int_field(o, "seq"), p))
                })
                .collect()
        })
        .unwrap_or_default();
    projects.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let projects: Vec<Project> = projects.into_iter().map(|(_, _, p)| p).collect();

    // Skills retain the database ordering; experience is curated in source.
    let mut skills: Vec<(i64, i64, Skill)> = r
        .get(3)
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|o| {
                    let s: Skill = from_row(o, "skill")?;
                    Some((int_field(o, "display_order"), int_field(o, "seq"), s))
                })
                .collect()
        })
        .unwrap_or_default();
    skills.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let skills: Vec<Skill> = skills.into_iter().map(|(_, _, s)| s).collect();

    Ok(PortfolioData {
        about,
        summary,
        projects: current_projects(projects),
        experiences: current_experiences(),
        skills,
    })
}

#[cfg(feature = "ssr")]
fn int_field(o: &serde_json::Value, k: &str) -> i64 {
    o.get(k).and_then(|v| v.as_i64()).unwrap_or(0)
}

// Deserialize one row, logging (rather than silently dropping) a malformed one so a
// type/shape mismatch surfaces in the logs instead of a record vanishing from the page.
#[cfg(feature = "ssr")]
fn from_row<T: serde::de::DeserializeOwned>(o: &serde_json::Value, kind: &str) -> Option<T> {
    match serde_json::from_value::<T>(o.clone()) {
        Ok(v) => Some(v),
        Err(e) => {
            log::warn!("skipping malformed {kind} row: {e}");
            None
        }
    }
}
