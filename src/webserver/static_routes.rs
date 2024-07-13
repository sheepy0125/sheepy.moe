//! Static routes

use crate::webserver::constants::{DIST_DIRECTORY, STATIC_DIRECTORY};

use const_format::formatcp;
use std::path::Path;
use warp::{reject::Rejection, reply::Reply, Filter};

/// Trunk dist routes
pub fn dist_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("static").and(warp::path("dist")).and({
        let dist_path = Path::new(DIST_DIRECTORY)
            .canonicalize()
            .expect(formatcp!("could not canonicalize {DIST_DIRECTORY}"));
        warp::fs::dir(dist_path)
    })
}

/// Other static routes
pub fn static_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("static").and({
        let static_directory = Path::new(STATIC_DIRECTORY)
            .canonicalize()
            .expect(formatcp!("could not canonicalize {STATIC_DIRECTORY}"));
        warp::fs::dir(static_directory)
    })
}
