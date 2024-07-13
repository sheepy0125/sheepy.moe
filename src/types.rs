//! Shared types between the client and server side rendering

use std::ops::Deref;

use bounce::Atom;
use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use yew::Properties;

use crate::router::Route;

/// Dynamic page state updated by CSR
#[derive(Atom, PartialEq, Clone, Debug, Properties, Default)]
pub struct PageState {
    pub initialized: bool,
    pub route: Option<String>,
    pub yew_route: Option<Route>,
}
impl PageState {
    pub fn new(route: Option<String>) -> Self {
        Self {
            route,
            initialized: true,
            yew_route: None,
        }
    }
}
/// Static page state from initial SSR load
#[derive(Atom, PartialEq, Clone, Debug, Properties, Default, Serialize, Deserialize)]
pub struct SSRState {
    pub count: usize,
    pub config: Configuration,
    pub route: String,
}

/// 88x31 button information
#[derive(Deserialize)]
pub struct Button88x31 {
    pub src: Box<str>,
    pub alt: Box<str>,
    pub href: Box<str>,
}

#[derive(Deserialize, Serialize, Default, PartialEq, Copy, Clone, Debug, strum_macros::Display)]
pub enum Theme {
    #[strum(to_string = "light")]
    Light,
    #[default]
    #[strum(to_string = "dark")]
    Dark,
}
impl Theme {
    pub fn opposite(&self) -> Self {
        match *self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[derive(Atom, Default, Deserialize, Serialize, PartialEq)]
pub struct Counter(usize);
impl Deref for Counter {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Counter {
    pub fn new(count: usize) -> Self {
        Self(count)
    }
}

// FIXME: update in future would mean unserialized fields in cookies
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Atom)]
pub struct Configuration {
    pub theme: Theme,
    pub font: Font,
    pub toaster: bool,
    pub no_textured_background: bool,
    pub show_banner: bool,
}
impl Default for Configuration {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            font: Default::default(),
            show_banner: false,
            toaster: true,
            no_textured_background: false,
        }
    }
}
impl Configuration {
    pub fn classes(&self) -> String {
        format!(
            "{theme} font-{font} textured-background-{textured_background}",
            theme = self.theme,
            font = self.font,
            textured_background = !self.no_textured_background
        )
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash, strum_macros::Display,
)]
pub enum Font {
    #[default]
    #[strum(to_string = "roboto-serif")]
    RobotoSerif,
    #[strum(to_string = "open-dyslexic")]
    OpenDyslexic,
    #[strum(to_string = "comic-sans")]
    ComicSans, // obviously the best font /s
}
impl ImplicitClone for Font {}
