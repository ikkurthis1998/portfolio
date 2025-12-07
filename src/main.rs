#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use portfolio::App;
    use tower_http::services::ServeDir;

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .leptos_routes(&leptos_options, routes, {
            let opts = leptos_options.clone();
            move || shell(opts.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(feature = "ssr")]
fn shell(_opts: leptos::config::LeptosOptions) -> impl leptos::prelude::IntoView {
    use leptos::prelude::*;
    use portfolio::App;
    view! { <App/> }
}

#[cfg(not(feature = "ssr"))]
fn main() {}
