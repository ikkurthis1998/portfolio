use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub language: String,
    pub stars: u32,
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
    pub projects: Vec<Project>,
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
}

pub fn get_portfolio_data() -> PortfolioData {
    const DATA: &str = include_str!("../assets/data.json");
    serde_json::from_str(DATA).expect("Invalid data.json")
}
