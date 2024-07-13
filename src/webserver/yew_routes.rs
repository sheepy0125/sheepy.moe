//! Yew.rs SSR routes

use crate::{
    constants::CONFIG_COOKIE_KEY,
    types::{Configuration, PageState, SSRState},
    webserver::{
        constants::SENTINEL_HTML_CLASS, counter_routes::GLOBAL_COUNTER, log::get_logger,
        server_side_render::reply_with_yew_ssr,
    },
};

use lazy_static::lazy_static;
use std::sync::atomic::Ordering;
use urldecode::decode;
use warp::{filters::path::FullPath, reject::Rejection, reply::Reply, Filter};

use super::{constants::SENTINEL_PAGE_STATE, util::reply_with_update_cookie};

/// Filter for accepting a thread-safe value in a handler
pub fn with<T>(something: T) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: Clone + Sync + Send,
{
    warp::any()
        .map(move || something.clone())
        .and_then(|something: T| async move { Ok::<_, warp::Rejection>(something) })
}

struct HtmlContent<'a> {
    pub pre_html_class: &'a str,
    pub post_html_class_pre_ssr_state: &'a str,
    pub post_ssr_state_pre_ssr: &'a str,
    pub post_ssr: &'a str,
}

fn get_html_content(file_content: &str) -> HtmlContent {
    let (pre_html_class, remaining) = file_content
        .split_once(SENTINEL_HTML_CLASS)
        .expect("failed to find placeholder theme");
    let (post_html_class_pre_ssr_state, remaining) = remaining
        .split_once(SENTINEL_PAGE_STATE)
        .expect("failed to find placeholder config");
    let (post_ssr_state_pre_ssr, remaining) = remaining
        .split_once("<body>")
        .expect("failed to find body opening");
    let (_, post_ssr) = remaining
        .split_once("</body>")
        .expect("failed to find body closing");
    HtmlContent {
        pre_html_class,
        post_html_class_pre_ssr_state,
        post_ssr_state_pre_ssr,
        post_ssr,
    }
}

lazy_static! {
    static ref HTML_FILE: String = std::fs::read_to_string("dist/index.html").unwrap();
    static ref HTML_CONTENT: HtmlContent<'static> = get_html_content(&HTML_FILE);
}

/// Warp to Yew.rs SSR
pub fn yew_ssr() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    #[cfg(not(debug_assertions))]
    let (pre_html_class, post_html_class_pre_ssr_state, post_ssr_state_pre_ssr, post_ssr) = {
        (
            HTML_CONTENT.pre_html_class,
            HTML_CONTENT.post_html_class_pre_ssr_state,
            HTML_CONTENT.post_ssr_state_pre_ssr,
            HTML_CONTENT.post_ssr,
        )
    };

    warp::any()
        .and(warp::path::full())
        .and(warp::cookie::optional::<String>(CONFIG_COOKIE_KEY))
        .and_then(move |route: FullPath, config_str: Option<String>| {
            let config_str = config_str.clone().map(decode);
            let config = config_str
                .clone()
                .and_then(|config_str| ron::from_str::<Configuration>(&config_str).ok())
                .unwrap_or_default();
            let count = GLOBAL_COUNTER.load(Ordering::Relaxed);

            let ssr_state = SSRState {
                route: route.as_str().to_string(),
                count,
                config,
            };

            async move {
                // Reload index HTML in debug environment
                #[cfg(debug_assertions)]
                let file_content = tokio::fs::read_to_string("dist/index.html").await.unwrap();
                #[cfg(debug_assertions)]
                let HtmlContent {
                    pre_html_class,
                    post_html_class_pre_ssr_state,
                    post_ssr_state_pre_ssr,
                    post_ssr,
                } = get_html_content(&file_content);

                let ssr_state_str = ron::to_string(&ssr_state).unwrap();
                let config = ssr_state.config.clone();
                let ssr =
                    reply_with_yew_ssr(PageState::new(Some(route.as_str().to_string())), ssr_state)
                        .await;

                let resp = format!(
                    "{pre_html_class}{classes}\
                    {post_html_class_pre_ssr_state}{ssr_state}{post_ssr_state_pre_ssr}\
                    <body>{ssr}</body>{post_ssr}",
                    classes = config.classes(),
                    ssr_state = ssr_state_str,
                );

                let resp = warp::reply::html(resp);
                Ok::<_, Rejection>(reply_with_update_cookie(
                    CONFIG_COOKIE_KEY,
                    &config_str
                        .unwrap_or_else(|| ron::to_string(&Configuration::default()).unwrap()),
                    resp,
                ))
            }
        })
        .with(warp::log::custom(get_logger))
}
