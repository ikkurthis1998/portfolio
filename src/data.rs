use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
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
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
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
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Skill {
    pub name: String,
    pub level: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
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
    let pool = get_db().await?;

    // Fetch About & Summary
    let (about, summary): (String, Option<String>) = sqlx::query_as("SELECT about, summary FROM profile LIMIT 1")
        .fetch_one(&pool)
        .await?;
    let summary = summary.unwrap_or_else(|| about.clone());

    // Fetch Projects
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY display_order ASC, id ASC")
        .fetch_all(&pool)
        .await?;

    // Fetch Experiences
    let experiences = sqlx::query_as::<_, Experience>("SELECT * FROM experiences ORDER BY start_date DESC NULLS LAST, id ASC")
        .fetch_all(&pool)
        .await?;

    // Fetch Skills
    let skills = sqlx::query_as::<_, Skill>("SELECT * FROM skills ORDER BY display_order ASC, id ASC")
        .fetch_all(&pool)
        .await?;

    Ok(PortfolioData {
        about,
        summary,
        projects,
        experiences,
        skills,
    })
}

