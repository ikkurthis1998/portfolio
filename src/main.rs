#[cfg(feature = "ssr")]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use portfolio::App;
    use tower_http::services::ServeDir;

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .leptos_routes(&leptos_options, routes, {
            let opts = leptos_options.clone();
            move || shell(opts.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    Ok(router.into())
}

#[cfg(feature = "ssr")]
fn shell(_opts: leptos::config::LeptosOptions) -> impl leptos::prelude::IntoView {
    use leptos::prelude::*;
    use portfolio::App;
    view! { <App/> }
}

#[cfg(not(feature = "ssr"))]
fn main() {}
