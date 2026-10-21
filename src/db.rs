// SurrealDB data layer for the portfolio (replaces the former sqlx/Postgres path).
//
// Talks to SurrealDB's HTTP `/sql` endpoint with a thin reqwest client. This is
// deliberately version-agnostic: the box runs SurrealDB v3, and going over HTTP
// sidesteps any Rust-SDK-vs-server protocol mismatch. All portfolio data lives in
// its own namespace/database (default portfolio/portfolio) on the shared instance.
use serde::Deserialize;
use leptos::prelude::*;

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
/// connect-on-start behaviour, but does not panic — the visit-recording path
/// fails soft so the static site still serves if SurrealDB is briefly down).
pub async fn init_db(base: &str, ns: &str, db: &str, user: &str, pass: &str) -> SurrealClient {
    let client = SurrealClient::new(base, ns, db, user, pass);
    match client.health().await {
        Ok(_) => log::info!("Connected to SurrealDB at {base} (ns={ns} db={db})"),
        Err(e) => log::error!("SurrealDB health check failed at boot: {e}"),
    }
    // Ensure the visit table exists (idempotent, best-effort) so record_visit
    // works on a fresh database.
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
