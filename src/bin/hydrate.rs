//! Client side Yew.rs hydration after the server side content has been served

use sheepy_moe::app::ClientApp;

use log::Level as LogLevel;
use wasm_logger::{init as log_init, Config as LogConfig};

fn main() {
    log_init(LogConfig::new(LogLevel::Trace));
    yew::Renderer::<ClientApp>::new().hydrate();
}
