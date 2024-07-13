//! "sheepy.moe" server-side-rendered + hydrated website, made with Yew.rs!!
//! See `package.json` for the run/build scripts. Running `yarn` and then `yarn dev` should be all!

pub mod app;
pub mod links;
pub mod router;
pub mod types;

pub mod components;
pub mod routes;
pub mod services;

pub mod constants;

#[cfg(not(target_arch = "wasm32"))]
pub mod webserver;
