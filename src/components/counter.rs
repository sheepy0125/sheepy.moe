use crate::{
    links::{COUNT_INCREMENT_REDIRECT, SENTINEL},
    types::PageState,
};

use bounce::use_atom;
use gloo_net::http::Request;
use std::time::Duration;
use wasm_bindgen::UnwrapThrowExt as _;
use wasm_bindgen_futures::spawn_local;
use wasmtimer::{std::SystemTime, tokio::sleep};
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter_component() -> Html {
    let page_state = use_atom::<PageState>();
    let counter = use_atom::<crate::types::Counter>();

    let counter_node = use_node_ref();
    let animation_node = use_node_ref();

    let on_increment_count = {
        let counter = counter.clone();
        let animation_node = animation_node.clone();
        let counter_node = counter_node.clone();
        Callback::from(move |_: MouseEvent| {
            let counter_node = counter_node.clone();
            let animation_node = animation_node.clone();
            let counter = counter.clone();

            counter.set(crate::types::Counter::new(**counter + 1));

            // Disable counter for the meantime
            counter_node
                .cast::<HtmlElement>()
                .unwrap_throw()
                .style()
                .set_property("pointer-events", "none")
                .unwrap_throw();

            spawn_local(async move {
                let resp = Request::get("/count-increment").send().await;
                if let Ok(poggers) = resp {
                    let text = poggers.text().await.ok();
                    let count = text.and_then(|text| text.parse::<usize>().ok());
                    if let Some(count) = count {
                        counter.set(crate::types::Counter::new(count));
                    }
                }

                let animation_wrapper = animation_node.cast::<HtmlElement>().unwrap_throw();

                animation_wrapper
                    .style()
                    .set_property("display", "block")
                    .unwrap_throw();

                // Cooldown animation. I can't use `wasmtimer::interval` nor `wasmtimer::sleep`
                // "typically," because they produce really inconsistent results. I can hypothesize
                // that it has something to do with polling to not be ready a lot, or something,
                // and for the task to be a lower priority. Nonetheless, this hacky workaround of
                // sleeping for 0ms in a loop that checks if a next deltatime poll should happen
                // is the only thing I could get working. If I wanted to make this not hacky, I'd
                // need to use JavaScript I reckon.

                const DELAY_TIME_MS: u64 = 2000;
                const DELTATIME_MS: u64 = 1;
                const STEPS: u64 = DELAY_TIME_MS / DELTATIME_MS;

                let start = SystemTime::now();

                for step in 1..=STEPS {
                    let percent = (step * 100) / STEPS;

                    animation_wrapper
                        .style()
                        .set_property("width", &format!("{percent}%"))
                        .unwrap_throw();

                    // This is stupid, but works.
                    while SystemTime::now()
                        .duration_since(start)
                        .map_err(|_| ())
                        .unwrap_throw()
                        .as_millis()
                        < (DELTATIME_MS * step) as u128
                    {
                        // Serves the same purpose as `yield_now().await`
                        sleep(Duration::from_millis(0)).await;
                    }
                }

                // Re-enable counter element and mask cooldown
                counter_node
                    .cast::<HtmlElement>()
                    .unwrap_throw()
                    .style()
                    .set_property("pointer-events", "auto")
                    .unwrap_throw();
                animation_wrapper
                    .style()
                    .set_property("display", "none")
                    .unwrap_throw();
            });
        })
    };

    html! {
        <a
            class={
                format!(
                    "counter not-link flex {}",
                    if **counter < 10 {"flex-row px-2"} else {"flex-col text-center"}
                )
            }
            href={
                if cfg!(not(target_arch = "wasm32")) {
                    format!(
                        "{COUNT_INCREMENT_REDIRECT}?redirect={route}",
                        route=page_state.route.clone().unwrap_or_default()
                    )
                } else {
                    SENTINEL.to_string()
                }
            }
            ref={counter_node}
            onclick={on_increment_count}
        >
            <div class="counter-cooldown" ref={animation_node}></div>
            <h2 class="hidden lg:block my-auto pr-2">{"counter!"}</h2>
            <span class="count">{format!("{}+", **counter)}</span>
        </a>
    }
}
