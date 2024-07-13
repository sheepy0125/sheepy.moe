//! Theme change routes

use crate::{
    constants::CONFIG_COOKIE_KEY,
    types::{Configuration, Theme},
    webserver::{
        log::get_logger,
        util::{redirect_sanitize, reply_with_update_cookie},
    },
};

use std::collections::HashMap;
use warp::{
    reject::{reject, Rejection},
    reply::Reply,
    Filter,
};

pub fn theme_change_redirect() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("save-theme-change")
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        // Set as required since this is a no-op without cookies
        .and(warp::cookie(CONFIG_COOKIE_KEY))
        .and_then(
            |theme: String, queries: HashMap<String, String>, config: String| async move {
                let theme = ron::from_str::<Theme>(&theme).map_err(|_| reject())?;
                let mut config = ron::from_str::<Configuration>(&config).unwrap_or_default();
                config.theme = theme;
                Ok::<_, Rejection>(reply_with_update_cookie(
                    CONFIG_COOKIE_KEY,
                    &config,
                    warp::redirect::temporary(redirect_sanitize(queries)),
                ))
            },
        )
        .with(warp::log::custom(get_logger))
}
