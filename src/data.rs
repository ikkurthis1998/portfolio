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

#[server(FetchPortfolioData, "/api")]
pub async fn fetch_portfolio_data() -> Result<PortfolioData, ServerFnError> {
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

    // experiences — order by start_date desc (NULLs last), then seq asc
    let mut experiences: Vec<(Option<String>, i64, Experience)> = r
        .get(2)
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|o| {
                    let e: Experience = from_row(o, "experience")?;
                    let start = o
                        .get("start_date")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    Some((start, int_field(o, "seq"), e))
                })
                .collect()
        })
        .unwrap_or_default();
    experiences.sort_by(|a, b| match (&a.0, &b.0) {
        (Some(x), Some(y)) => y.cmp(x).then(a.1.cmp(&b.1)), // ISO dates: string cmp = chrono
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.1.cmp(&b.1),
    });
    let experiences: Vec<Experience> = experiences.into_iter().map(|(_, _, e)| e).collect();

    // skills — order by display_order asc, then seq asc
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
        projects,
        experiences,
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
