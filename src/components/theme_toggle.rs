//! Theme toggle component

use crate::{
    components::heroicons::{Icon, IconType},
    links::{SAVE_THEME_CHANGE, SENTINEL},
    types::{Configuration, PageState, Theme},
};

use bounce::{use_atom, CloneAtom as _};
use wasm_bindgen::UnwrapThrowExt as _;
use yew::prelude::*;

#[function_component(ThemeToggle)]
pub fn theme_toggle_component() -> Html {
    let page_state = use_atom::<PageState>();
    let config_state = use_atom::<Configuration>();

    let theme_variant = use_state_eq(|| config_state.theme);

    let on_toggle_theme = {
        let theme_variant = theme_variant.clone();
        Callback::from(move |_: MouseEvent| {
            let opposite = Theme::opposite(&theme_variant);
            theme_variant.set(opposite);
        })
    };
    {
        let config_state = config_state.clone();
        use_effect_with(*theme_variant, move |theme| {
            // Update page state
            let mut config = config_state.clone_atom();
            config.theme = *theme;
            config_state.set(config);
        });
    }

    html! {
        <a
            class="theme-icon not-link"
            href={
                if cfg!(not(target_arch = "wasm32")) {
                    format!(
                        "{SAVE_THEME_CHANGE}/{opposite_theme}?redirect={route}",
                        opposite_theme=ron::to_string(&Theme::opposite(&config_state.theme)).unwrap_throw(),
                        route=page_state.route.clone().unwrap_or_default()
                    )
                } else {
                    SENTINEL.to_string()
                }
            }
            onclick={on_toggle_theme}
         >
            <Icon icon={match *theme_variant {
                Theme::Light => IconType::LightBulbLight,
                Theme::Dark => IconType::LightBulbDark,
           }} />
        </a>
    }
}
