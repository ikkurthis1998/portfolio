#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::middleware;
    use axum::Router;
    use std::net::SocketAddr;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use portfolio::App;
    use tower_http::services::ServeDir;
    use portfolio::db::{self, AppState};

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Initialize Database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = db::init_db(&db_url).await;
    let app_state = AppState { db: db_pool.clone() };

    // build our application with a route
    let app = Router::new()
        // .nest_service("/assets", ServeDir::new("assets")) // Assets are now CDN served
        .leptos_routes(&leptos_options, routes, {
            let opts = leptos_options.clone();
            move || shell(opts.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .route("/health", axum::routing::get(health_check))
        .layer(middleware::from_fn_with_state(app_state.clone(), track_visitor))
        .layer(axum::Extension(app_state.clone()))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn track_visitor(
    axum::extract::State(state): axum::extract::State<portfolio::db::AppState>,
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {

    let headers = req.headers().clone();
    
    // Insert DB into extensions so server functions can access it
    req.extensions_mut().insert(state.db.clone());

    // Extract info
    let path = req.uri().path().to_string();
        
    // Skip api/metrics/favicon
    if !path.starts_with("/api") && !path.contains("favicon") {
        let domain = headers
            .get("host")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let country = headers
            .get("cf-ipcountry")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("XX")
            .to_string();

        let user_agent = headers
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        // IP is tricky behind proxies. Cloudflare sends CF-Connecting-IP
        let ip = headers
            .get("cf-connecting-ip")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                 // Fallback to direct connection info if available, or placeholder
                 "127.0.0.1".to_string() 
            });

        let db = state.db.clone();
        
        // Spawn async task to record visit so we don't block the request
        tokio::spawn(async move {
            portfolio::db::record_visit(&db, ip, path, domain, country, user_agent).await;
        });
    }

    next.run(req).await
}

#[cfg(feature = "ssr")]
async fn health_check(axum::Extension(state): axum::Extension<portfolio::db::AppState>) -> impl axum::response::IntoResponse {
    use axum::http::StatusCode;
    
    // Perform a simple query to assert DB connection is alive
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(e) => {
            log::error!("Health check failed: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Connection Failed")
        }
    }
}

#[cfg(feature = "ssr")]
fn shell(opts: leptos::config::LeptosOptions) -> impl leptos::prelude::IntoView {
    use leptos::prelude::*;
    use leptos_meta::*;
    use portfolio::App;
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=opts.clone().into() />
                <HydrationScripts options=opts.into() />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
