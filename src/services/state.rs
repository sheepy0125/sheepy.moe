use crate::{
    constants::CONFIG_COOKIE_KEY,
    types::{Configuration, SSRState},
};

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt as _;
use wasm_cookies::cookies::{set as cookies_set, CookieOptions};
use web_sys::{window, HtmlDocument};

pub fn get_initial_ssr_state() -> SSRState {
    let window = window().unwrap_throw();
    window
        .get("ssr_state")
        .and_then(|state_str| {
            ron::from_str(&state_str.as_string().expect("window.ssr_state not string")).ok()
        })
        .unwrap_or_default()
}

pub fn update_config_html_class(config: &Configuration) {
    let window = window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let document_element = document.document_element().unwrap_throw();

    document_element.set_class_name(&config.classes());
}

pub fn save_config(config: &Configuration) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<HtmlDocument>().unwrap();

    let cookies = cookies_set(
        CONFIG_COOKIE_KEY,
        &ron::to_string(config).unwrap(),
        &CookieOptions::default().expires_at_date("Tue, 19 Jan 2038 03:14:07 UTC"),
    );

    html_document.set_cookie(&cookies).unwrap();
}
