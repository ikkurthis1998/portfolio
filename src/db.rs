// SurrealDB data layer for the portfolio (replaces the former sqlx/Postgres path).
//
// Talks to SurrealDB's HTTP `/sql` endpoint with a thin reqwest client. This is
// deliberately version-agnostic: the box runs SurrealDB v3, and going over HTTP
// sidesteps any Rust-SDK-vs-server protocol mismatch. All portfolio data lives in
// its own namespace/database (default portfolio/portfolio) on the shared instance.
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use leptos::prelude::*;

// ── public data shapes consumed by the analytics page (unchanged contract) ──

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisitRecord {
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

// ── SurrealDB HTTP client ──

#[derive(Clone)]
pub struct SurrealClient {
    http: reqwest::Client,
    url: String,
    ns: String,
    db: String,
    user: String,
    pass: String,
}

#[derive(Deserialize)]
struct SqlResp {
    status: String,
    // Some statement types omit `result`; default to Null so one such statement
    // does not collapse the whole batch decode.
    #[serde(default)]
    result: serde_json::Value,
}

impl SurrealClient {
    pub fn new(base: &str, ns: &str, db: &str, user: &str, pass: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            url: format!("{}/sql", base.trim_end_matches('/')),
            ns: ns.to_string(),
            db: db.to_string(),
            user: user.to_string(),
            pass: pass.to_string(),
        }
    }

    /// Runs a batch of SurrealQL statements and returns one result value per
    /// statement (in order). Errors if any statement reports a non-OK status.
    pub async fn query(&self, sql: &str) -> Result<Vec<serde_json::Value>, String> {
        let resp = self
            .http
            .post(&self.url)
            .basic_auth(&self.user, Some(&self.pass))
            .header("surreal-ns", &self.ns)
            .header("surreal-db", &self.db)
            .header("Accept", "application/json")
            .body(sql.to_string())
            .send()
            .await
            .map_err(|e| format!("surreal request failed: {e}"))?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("surreal HTTP {code}: {body}"));
        }
        let stmts: Vec<SqlResp> = resp
            .json()
            .await
            .map_err(|e| format!("surreal decode failed: {e}"))?;
        let mut out = Vec::with_capacity(stmts.len());
        for s in stmts {
            if s.status != "OK" {
                return Err(format!("surreal statement error: {}", s.result));
            }
            out.push(s.result);
        }
        Ok(out)
    }

    pub async fn health(&self) -> Result<(), String> {
        self.query("RETURN 1;").await.map(|_| ())
    }
}

/// JSON-encode a string into a SurrealQL string literal. JSON escaping (\" \\ \n …)
/// yields a valid SurrealQL double-quoted string, which makes header-derived,
/// attacker-controlled visit fields injection-proof.
fn lit(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
}

#[derive(Clone)]
pub struct AppState {
    pub db: SurrealClient,
}

/// Builds the client and verifies connectivity at boot (mirrors the old
/// connect-on-start behaviour, but does not panic — the analytics/visit paths
/// fail soft so the static site still serves if SurrealDB is briefly down).
pub async fn init_db(base: &str, ns: &str, db: &str, user: &str, pass: &str) -> SurrealClient {
    let client = SurrealClient::new(base, ns, db, user, pass);
    match client.health().await {
        Ok(_) => log::info!("Connected to SurrealDB at {base} (ns={ns} db={db})"),
        Err(e) => log::error!("SurrealDB health check failed at boot: {e}"),
    }
    // Ensure the visit table exists. SurrealDB v3 errors ("table does not exist")
    // when selecting a never-written table, which would make /analytics render an
    // error instead of zeros on a fresh database. Idempotent + best-effort.
    if let Err(e) = client.query("DEFINE TABLE IF NOT EXISTS visit SCHEMALESS;").await {
        log::warn!("could not ensure visit table exists: {e}");
    }
    client
}

pub async fn get_db() -> Result<SurrealClient, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    let Extension(client): Extension<SurrealClient> = extract().await?;
    Ok(client)
}

pub async fn record_visit(
    db: &SurrealClient,
    ip: String,
    path: String,
    domain: String,
    country: String,
    ua: String,
) {
    // created_at via time::now() (real datetime); string fields JSON-escaped (safe).
    let sql = format!(
        "CREATE visit SET created_at = time::now(), ip_address = {}, path = {}, domain = {}, country = {}, user_agent = {};",
        lit(&ip),
        lit(&path),
        lit(&domain),
        lit(&country),
        lit(&ua),
    );
    if let Err(e) = db.query(&sql).await {
        log::warn!("record_visit failed: {e}");
    }
}

pub async fn get_analytics(db: &SurrealClient) -> Result<AnalyticsStats, String> {
    // One round-trip; statement order matches the parsing below.
    let sql = r#"SELECT count() AS c FROM visit WHERE path != '/analytics' GROUP ALL;
RETURN count(array::distinct(SELECT VALUE ip_address FROM visit WHERE path != '/analytics'));
RETURN count(array::distinct(SELECT VALUE ip_address FROM visit WHERE path != '/analytics' AND created_at > time::now() - 5m));
SELECT country, count() AS c FROM visit WHERE path != '/analytics' GROUP BY country ORDER BY c DESC LIMIT 5;
SELECT domain, count() AS c FROM visit WHERE path != '/analytics' GROUP BY domain ORDER BY c DESC LIMIT 5;
SELECT * FROM visit WHERE path != '/analytics' ORDER BY created_at DESC LIMIT 20;"#;

    let r = db.query(sql).await?;

    let total_visitors = r
        .first()
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|o| o.get("c"))
        .and_then(|c| c.as_i64())
        .unwrap_or(0);
    let unique_visitors = r.get(1).and_then(|v| v.as_i64()).unwrap_or(0);
    let active_now = r.get(2).and_then(|v| v.as_i64()).unwrap_or(0);
    let top_countries = parse_pairs(r.get(3), "country");
    let top_domains = parse_pairs(r.get(4), "domain");
    let recent_visits = r
        .get(5)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(parse_visit).collect())
        .unwrap_or_default();

    Ok(AnalyticsStats {
        total_visitors,
        unique_visitors,
        active_now,
        top_countries,
        top_domains,
        recent_visits,
    })
}

fn parse_pairs(v: Option<&serde_json::Value>, key: &str) -> Vec<(String, i64)> {
    v.and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|o| {
                    let k = o.get(key)?.as_str()?.to_string();
                    let c = o.get("c")?.as_i64()?;
                    Some((k, c))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_visit(o: &serde_json::Value) -> Option<VisitRecord> {
    let created_raw = o.get("created_at")?.as_str()?;
    // Drop a row with an unparseable timestamp rather than fabricating Utc::now() —
    // a fabricated time would masquerade as the newest visit in the recent list.
    let created_at = DateTime::parse_from_rfc3339(created_raw)
        .ok()?
        .with_timezone(&Utc);
    let s = |k: &str| o.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
    Some(VisitRecord {
        created_at,
        ip_address: s("ip_address"),
        path: s("path"),
        domain: s("domain"),
        country: s("country"),
        user_agent: s("user_agent"),
    })
}
