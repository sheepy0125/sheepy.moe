use crate::{
    links::{COUNT_INCREMENT_REDIRECT, SENTINEL},
    types::PageState,
};

use bounce::use_atom;
use std::time::Duration;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use wasmtimer::{std::SystemTime, tokio::sleep};
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter_component() -> Html {
    let page_state = use_atom::<PageState>();
    let counter = use_atom::<crate::types::Counter>();

    let counter_node = use_node_ref();
    let cooldown_node = use_node_ref();

    let on_increment_count = {
        let counter = counter.clone();
        let cooldown_node = cooldown_node.clone();
        let counter_node = counter_node.clone();
        Callback::from(move |_: MouseEvent| {
            let counter = counter.clone();

            let counter_element = counter_node.cast::<HtmlElement>().unwrap_throw();
            let cooldown_element = cooldown_node.cast::<HtmlElement>().unwrap_throw();

            // Update counter to predicted value
            counter.set(crate::types::Counter::new(**counter + 1));

            spawn_local(async move {
                // Send request
                let resp = gloo_net::http::Request::get("/count-increment")
                    .send()
                    .await
                    .unwrap_throw();
                let text = resp.text().await.unwrap_throw();
                let count = text.parse::<usize>().unwrap_throw();
                counter.set(crate::types::Counter::new(count));

                // Counter cooldown
                counter_element
                    .style()
                    .set_property("pointer-events", "none")
                    .unwrap_throw();
                cooldown_element
                    .style()
                    .set_property("display", "block")
                    .unwrap_throw();

                // fixme: using [`wasmtimer::sleep`] produces inconsistent results.
                // this shouldn't be the case, since `spawn_local` polls on every microtask
                // and `setTimeout` is a macrotask. nonetheless, this hacky workaround of
                // sleeping for 0ms (which *should be* equivalent to `yield_now().await`
                // but the latter produces the same results) works. we do need to yield,
                // otherwise the page freezes.

                const DELAY_TIME_MS: u64 = 2000;
                const DELTATIME_MS: u64 = 20;
                const STEPS: u64 = DELAY_TIME_MS / DELTATIME_MS;

                let start = SystemTime::now();

                for step in 1..=STEPS {
                    let percent = (step * 100) / STEPS;

                    cooldown_element
                        .style()
                        .set_property("width", &format!("{percent}%"))
                        .unwrap_throw();

                    while SystemTime::now()
                        .duration_since(start)
                        .map_err(|_| ())
                        .unwrap_throw()
                        .as_millis()
                        < (DELTATIME_MS * step) as u128
                    {
                        sleep(Duration::from_millis(0)).await;
                    }
                }

                // Re-enable counter element and mask cooldown
                counter_element
                    .style()
                    .set_property("pointer-events", "auto")
                    .unwrap_throw();
                cooldown_element
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
            <div class="counter-cooldown" ref={cooldown_node}></div>
            <h2 class="hidden lg:block my-auto pr-2">{"counter!"}</h2>
            <span class="count">{format!("{}+", **counter)}</span>
        </a>
    }
}
