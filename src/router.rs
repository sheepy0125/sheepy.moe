//! All routes

use crate::{
    app::PAGE_RENDER_COUNT,
    routes::{about::About, blog::Blog, homepage::Homepage, settings::Settings},
};

use log::debug;
use std::{fmt::Display, sync::atomic::Ordering};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug, Routable, strum_macros::Display)]
pub enum Route {
    #[at("/home")]
    #[strum(to_string = "home")]
    Homepage,
    #[at("/blog")]
    #[strum(to_string = "blog")]
    BlogRoot,
    #[at("/blog/*")]
    #[strum(to_string = "blog")]
    BlogRoute,
    #[at("/about")]
    #[strum(to_string = "about")]
    About,
    #[at("/settings")]
    #[strum(to_string = "settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    #[strum(to_string = "not found LOL")]
    NotFound,
}
/// Hacky nested router to support dynamically nested paths
#[derive(Clone, Routable, PartialEq)]
pub enum BlogRoute {
    #[at("/blog/:a")]
    BlogA { a: String },
    #[at("/blog/:a/:b")]
    BlogAB { a: String, b: String },
    #[at("/blog/:a/:b/:c")]
    BlogABC { a: String, b: String, c: String },
    #[at("/blog/:a/:b/:c/:d")]
    BlogABCD {
        a: String,
        b: String,
        c: String,
        d: String,
    },
    #[at("/blog/:full_path")]
    BlogFullPath { full_path: String },
}
impl Display for BlogRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BlogA { a } => a.to_string(),
                Self::BlogAB { a, b } => format!("{a}/{b}"),
                Self::BlogABC { a, b, c } => format!("{a}/{b}/{c}"),
                Self::BlogABCD { a, b, c, d } => format!("{a}/{b}/{c}/{d}"),
                Self::BlogFullPath { full_path } => full_path.clone(),
            }
        )
    }
}
fn hacky_switch_blog(route: BlogRoute) -> Html {
    html! {<Blog post={route.to_string()} />}
}

pub const ITER_ROUTES: &[Route] = &[Route::Homepage, Route::BlogRoot, Route::Settings];

pub fn switch(route: Route) -> Html {
    // FIXME: there are multiple page renders on initial render
    let page_render = PAGE_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
    debug!("render #{} on {route}", page_render + 1);

    match route {
        Route::Homepage => html! {<Homepage />},
        Route::About => html! {<About />},
        Route::BlogRoot => html! {<Blog post={"index".to_string()} />},
        Route::BlogRoute => {
            html! {<Switch<BlogRoute> render={hacky_switch_blog} />}
        }
        Route::Settings => html! {<Settings />},
        Route::NotFound => html! {<a class="link" href="/home">{"not found lol!"}</a>},
    }
}
