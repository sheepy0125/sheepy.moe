//! Blog post routes

use crate::webserver::constants::BLOG_DIRECTORY;

use const_format::formatcp;
use highlight_pulldown::highlight_with_theme;
use pulldown_cmark::{html::push_html, Parser};
use std::path::{Path, PathBuf};
use warp::{filters::path::Tail, reject::Rejection, reply::Reply, Filter};

pub fn blog_post_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("raw_blog")
        .and(warp::path::tail())
        .and_then(move |tail: Tail| async move {
            let blog_directory = Path::new(BLOG_DIRECTORY)
                .canonicalize()
                .expect(formatcp!("could not canonicalize {BLOG_DIRECTORY}"));
            let post_path = match blog_directory
                .join(PathBuf::from(format!("{}.md", tail.as_str())))
                .canonicalize()
            {
                Ok(path) => path,
                Err(_) => {
                    return Ok::<_, warp::Rejection>(warp::reply::html("not found".to_string()))
                }
            };

            Ok::<_, warp::Rejection>(match tokio::fs::read_to_string(&post_path).await {
                Ok(post) => warp::reply::html(render_markdown(&post)),
                Err(_) => warp::reply::html("not found".to_string()),
            })
        })
}

pub fn render_markdown(src: &str) -> String {
    let mut html_buf = String::new();
    let events = Parser::new(src);
    let events = highlight_with_theme(events, "base16-ocean.dark").unwrap();
    push_html(&mut html_buf, events.into_iter());
    html_buf
}
