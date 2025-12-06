use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub language: String,
    pub stars: u32,
    pub url: String,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub duration: String,
    pub description: String,
    pub technologies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub name: String,
    pub level: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfolioData {
    pub projects: Vec<Project>,
    pub experiences: Vec<Experience>,
    pub skills: Vec<Skill>,
}

pub async fn fetch_portfolio_data() -> Result<PortfolioData, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // On server side (SSR), we can't use gloo-net.
        // We could implement reqwest here if we added it, but for now we'll 
        // return an error to let the client handle the fetch (CSR fallback).
        Err("Data fetching only supported on client".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    {
        // URL for the data source. 
        // TODO: Replace this with your actual remote URL (e.g. GitHub Raw Gist)
        // For local development/testing, we can use the file served from assets
        // let url = "https://raw.githubusercontent.com/username/repo/main/data.json";
        let url = "/assets/data.json"; 
        
        gloo_net::http::Request::get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<PortfolioData>()
            .await
            .map_err(|e| e.to_string())
    }
}
