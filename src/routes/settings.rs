use crate::{
    components::main_page_content::MainPageContent,
    router::Route,
    types::{Configuration, Font},
};

use bounce::{use_atom, CloneAtom as _};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[function_component(SettingsHandler)]
pub fn settings_handler_component() -> Html {
    let config = use_atom::<Configuration>();

    macro_rules! change_config {
        ($key:ident, $value:expr) => {{
            let config_state = config.clone();
            #[allow(unused_variables)]
            Callback::from(move |e: Event| {
                let mut config = config_state.clone_atom();
                config.$key = $value(e);
                config_state.set(config);
            })
        }};
    }

    html! {
        <form action="/save-settings-change" method="POST" >
            <fieldset>
                <legend>{"Website Appearance"}</legend>
                <label class="toggle-outer-label">
                    <input
                        type="checkbox"
                        name="textured-background"
                        class="toggle-input peer"
                        checked={!config.no_textured_background}
                        onchange={change_config!(no_textured_background, |e: Event| !e.target_dyn_into::<HtmlInputElement>().unwrap().checked())}
                    />
                    <div class="toggle-sibling peer"></div>
                    <span class="toggle-label">{"Textured wallpaper"}</span>
                </label>
                <br />
                <label class="toggle-outer-label">
                    <input
                        type="checkbox"
                        name="show-banner"
                        class="toggle-input peer"
                        checked={config.show_banner}
                        onchange={change_config!(show_banner, |e: Event| e.target_dyn_into::<HtmlInputElement>().unwrap().checked())}
                    />
                    <div class="toggle-sibling peer"></div>
                    <span class="toggle-label">{"Show banner on pages other than homepage"}</span>
                </label>
                <div class="block">
                    <label for="font-selection">{"Font "}</label>
                    <select id="font-selection" name="font" onchange={
                        change_config!(font, |e: Event| ron::from_str(&e.target_dyn_into::<HtmlSelectElement>().unwrap().value()).unwrap())
                    }>
                        <option value={ron::to_string(&Font::RobotoSerif).unwrap()} selected={config.font == Font::RobotoSerif}>{"Roboto Serif (default)"}</option>
                        <option value={ron::to_string(&Font::OpenDyslexic).unwrap()} selected={config.font == Font::OpenDyslexic}>{"Open Dyslexic"}</option>
                        <option value={ron::to_string(&Font::ComicSans).unwrap()} selected={config.font == Font::ComicSans}>{"Comic Sans MS"}</option>
                    </select>
                </div>
            </fieldset>
            <input type="submit" value={
                if cfg!(target_arch = "wasm32") {
                    "Force save changes in cookies"
                } else {
                    "Save changes (you don't have WebAssembly!)"
                }
            }
            class="link pointer-hover" />
        </form>
    }
}

#[function_component(Settings)]
pub fn settings_component() -> Html {
    html! {
        <MainPageContent page={Route::Settings} first_widget={html! {
            <section id="settings" class="widget">
                <div class="widget-banner">
                    <h1 class="section-header">{"settings"}</h1>
                </div>
                <SettingsHandler />
            </section>
        }}>
            <></>
        </MainPageContent>
    }
}
