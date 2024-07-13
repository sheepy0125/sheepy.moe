//! Settings change route

use crate::{
    constants::CONFIG_COOKIE_KEY,
    types::Configuration,
    webserver::{log::get_logger, util::reply_with_update_cookie},
};

use std::collections::HashMap;
use urldecode::decode;
use warp::{http::Uri, reject::Rejection, reply::Reply, Filter};

pub fn settings_update() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("save-settings-change")
        .and(warp::post())
        .and(warp::body::form())
        // Set as required since this is a no-op without cookies
        .and(warp::cookie(CONFIG_COOKIE_KEY))
        .and_then(
            |form: HashMap<String, String>, current_config: String| async move {
                let config_str = decode(current_config);
                let mut config = ron::from_str::<Configuration>(&config_str).unwrap_or_default();
                config.no_textured_background = !form
                    .get("textured-background")
                    .map(|str| str == "on")
                    .unwrap_or(false);
                config.show_banner = form.get("show-banner").map(|str| str == "on").unwrap_or(false);
                config.font = form
                    .get("font")
                    .and_then(|font_str| ron::from_str(font_str).ok())
                    .unwrap_or_default();
                Ok::<_, Rejection>(reply_with_update_cookie(
                    CONFIG_COOKIE_KEY,
                    &config,
                    warp::redirect::temporary(Uri::from_static("/settings")),
                ))
            },
        )
        .with(warp::log::custom(get_logger))
}
