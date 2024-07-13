use log::info;
use warp::filters::log::Info;

pub fn get_logger(info: Info) {
    if info.status() == warp::http::StatusCode::NOT_FOUND {
        return;
   }

    info!(
        "{}: {} to {} returned {} and took {}ms",
        info.request_headers()
            .get("X-Real-IP")
            .map_or_else(
                || info.remote_addr().map(|address| address.to_string()),
                |address| address.to_str().ok().map(|address| address.to_string()),
            )
            .unwrap_or_else(|| "unknown".to_string()),
        info.method(),
        info.path(),
        info.status(),
        info.elapsed().as_millis()
    );
}
