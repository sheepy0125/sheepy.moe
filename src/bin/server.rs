//! "sheepy.moe" website server!

use sheepy_moe::webserver::{
    blog_post_routes::blog_post_routes,
    counter_routes::{count_increment, count_increment_redirect, counter_load_save_loop},
    settings::settings_update,
    static_routes::{dist_routes, static_routes},
    theme_change_routes::theme_change_redirect,
    yew_routes::yew_ssr,
};

use dotenvy::dotenv;
use jemallocator::Jemalloc;
use log::info;
use std::io::Write;
use tokio::spawn;
use warp::{http::Uri, Filter as _};

#[cfg(unix)]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    dotenv().expect("failed to load environment variables");
    let port = dotenvy::var("PORT").expect("failed to find PORT env");
    let port = port.parse().expect("failed to parse PORT env");

    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    )
    .format(|buf, record| {
        let style = buf.default_level_style(record.level());
        let styled_timestamp = style.value(buf.timestamp());
        writeln!(buf, "[{styled_timestamp}] {}", record.args())
    })
    .init();

    spawn(counter_load_save_loop());

    let home_redirect =
        || warp::path::end().map(|| warp::redirect::permanent(Uri::from_static("/home")));

    let routes = dist_routes()
        .or(static_routes())
        .or(blog_post_routes())
        .or(count_increment().or(count_increment_redirect()))
        .or(theme_change_redirect())
        .or(settings_update())
        .or(home_redirect())
        // This includes the 404 handler!
        .or(yew_ssr());

    // Run Warp server
    info!("running on port {port}!");
    warp::serve(routes).run(([0; 4], port)).await;
}
