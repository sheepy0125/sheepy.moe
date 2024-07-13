use serde::Serialize;
use std::collections::HashMap;
use warp::{http::Uri, reply::Reply};

/// Fetch the `?redirect` query parameter and return a URL, avoiding redirecting to other pages
pub fn redirect_sanitize(queries: HashMap<String, String>) -> Uri {
    let uri = queries
        .get("redirect")
        .and_then(|path| Uri::try_from(path.to_owned()).ok())
        .unwrap_or_else(|| Uri::from_static("/"));
    match uri.scheme() {
        Some(_) => Uri::from_static("/"),
        None => uri,
    }
}

/// Update configuration cookie
pub fn reply_with_update_cookie<V>(key: &str, value: &V, reply: impl Reply) -> impl Reply
where
    V: ?Sized + Serialize,
{
    warp::reply::with_header(
        reply,
        "set-cookie",
        format!(
            "{key}={value}; SameSite=Lax; Path=/",
            value = ron::to_string(value)
                .unwrap()
                .trim_start_matches('"')
                .trim_end_matches('"')
        ),
    )
}
