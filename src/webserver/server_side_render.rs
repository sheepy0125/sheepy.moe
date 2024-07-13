//! Server side render

use crate::{
    app::{ServerApp, ServerAppProps},
    types::{PageState, SSRState},
};

use yew::ServerRenderer;

/// Server-side-render a route
pub async fn reply_with_yew_ssr(state: PageState, ssr_state: SSRState) -> String {
    let mut resp = String::new();
    let renderer =
        ServerRenderer::<ServerApp>::with_props(move || ServerAppProps { state, ssr_state });
    renderer.render_to_string(&mut resp).await;
    resp
}
