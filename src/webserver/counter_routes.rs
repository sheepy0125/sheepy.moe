//! Global counter route

use crate::webserver::{constants::COUNT_FILE, log::get_logger, util::redirect_sanitize};

use std::{collections::HashMap, sync::atomic::AtomicUsize, sync::atomic::Ordering};
use warp::{reject::Rejection, reply::Reply, Filter};

pub static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub async fn save_count() {
    let count = GLOBAL_COUNTER.load(std::sync::atomic::Ordering::Relaxed);
    let count_str = format!("{count}");
    tokio::fs::write(COUNT_FILE, count_str.as_bytes())
        .await
        .unwrap();
}
pub async fn load_count() {
    let count = tokio::fs::read_to_string(COUNT_FILE)
        .await
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or_default();
    GLOBAL_COUNTER.swap(count, std::sync::atomic::Ordering::Relaxed);
}

pub fn count_increment() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("count-increment")
        .and(warp::path::end())
        .map(|| format!("{}", GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed) + 1))
        .with(warp::log::custom(get_logger))
}

pub fn count_increment_redirect() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("count-increment-redirect")
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .map(|queries: HashMap<String, String>| {
            let _ = GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed);
            warp::redirect::temporary(redirect_sanitize(queries))
        })
        .with(warp::log::custom(get_logger))
}
