use axum::Router;
use leptos_axum::generate_route_list;
use portfolio::App;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Build our application with a route
    let conf = leptos::config::get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let _routes = generate_route_list(App);

    let app = Router::new()
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .nest_service("/static", ServeDir::new("target/site/static"))
        .nest_service("/assets", ServeDir::new("target/site"))
        .fallback_service(ServeDir::new(".").append_index_html_on_directories(true));

    let listener = TcpListener::bind(&addr).await.unwrap();
    log::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
