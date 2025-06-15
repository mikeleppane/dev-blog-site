#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    use axum::Router;
    use color_eyre;
    use dotenv::dotenv;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use mike_dev_blog::app::*;
    use mike_dev_blog::services::config::init_config;
    use std::sync::Arc;
    use tracing_subscriber;
    use tracing_subscriber::fmt::init;

    color_eyre::install()?;

    // Initialize logging
    init();

    dotenv().ok();

    let app_config = init_config();
    let shared_config = Arc::new(app_config);

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .with_state(shared_config);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    Ok(axum::serve(listener, app.into_make_service())
        .await
        .unwrap())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
