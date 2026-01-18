use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use leptos::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VisitRecord {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub ip_address: String,
    pub path: String,
    pub domain: String,
    pub country: String,
    pub user_agent: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AnalyticsStats {
    pub total_visitors: i64,
    pub unique_visitors: i64,
    pub active_now: i64,
    pub top_countries: Vec<(String, i64)>,
    pub top_domains: Vec<(String, i64)>,
    pub recent_visits: Vec<VisitRecord>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

use std::str::FromStr;
use sqlx::postgres::PgConnectOptions;

pub async fn init_db(database_url: &str) -> Pool<Postgres> {
    let options = PgConnectOptions::from_str(database_url)
        .expect("Invalid connection string")
        .statement_cache_capacity(0);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to connect to Postgres");

    // Create table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS visits (
            id BIGSERIAL PRIMARY KEY,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            ip_address TEXT NOT NULL,
            path TEXT NOT NULL,
            domain TEXT NOT NULL,
            country TEXT NOT NULL,
            user_agent TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    pool
}

#[cfg(feature = "ssr")]
pub async fn get_db() -> Result<Pool<Postgres>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    
    let Extension(pool): Extension<Pool<Postgres>> = extract().await?;
    Ok(pool)
}

pub async fn record_visit(
    pool: &Pool<Postgres>,
    ip: String,
    path: String,
    domain: String,
    country: String,
    ua: String,
) {
    let _ = sqlx::query(
        r#"
        INSERT INTO visits (ip_address, path, domain, country, user_agent)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(ip)
    .bind(path)
    .bind(domain)
    .bind(country)
    .bind(ua)
    .execute(pool)
    .await;
}

pub async fn get_analytics(pool: &Pool<Postgres>) -> Result<AnalyticsStats, sqlx::Error> {
    let total_visitors: i64 = sqlx::query("SELECT COUNT(*) FROM visits WHERE path != '/analytics'")
        .fetch_one(pool)
        .await?
        .get(0);

    let unique_visitors: i64 = sqlx::query("SELECT COUNT(DISTINCT ip_address) FROM visits WHERE path != '/analytics'")
        .fetch_one(pool)
        .await?
        .get(0);

    let active_now: i64 = sqlx::query(
        "SELECT COUNT(DISTINCT ip_address) FROM visits WHERE path != '/analytics' AND created_at > NOW() - INTERVAL '5 minutes'"
    )
    .fetch_one(pool)
    .await?
    .get(0);

    let top_countries: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT country, COUNT(*) as c
        FROM visits
        WHERE path != '/analytics'
        GROUP BY country
        ORDER BY c DESC
        LIMIT 5
        "#
    )
    .fetch_all(pool)
    .await?;

    let top_domains: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT domain, COUNT(*) as c
        FROM visits
        WHERE path != '/analytics'
        GROUP BY domain
        ORDER BY c DESC
        LIMIT 5
        "#
    )
    .fetch_all(pool)
    .await?;

    let recent_visits = sqlx::query_as::<_, VisitRecord>(
        r#"
        SELECT 
            id,
            created_at,
            ip_address,
            path,
            domain,
            country,
            user_agent
        FROM visits
        WHERE path != '/analytics'
        ORDER BY created_at DESC
        LIMIT 20
        "#
    )
    .fetch_all(pool)
    .await?;


    Ok(AnalyticsStats {
        total_visitors,
        unique_visitors,
        active_now,
        top_countries,
        top_domains,
        recent_visits,
    })
}
