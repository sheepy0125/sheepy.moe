//! Client and server app entry points. Contains the router, too!

use crate::{
    router::{switch, Route},
    types::{Configuration, Counter, PageState, SSRState},
};

use bounce::{use_atom, BounceRoot};
use html::RenderError;
use log::debug;
use std::sync::atomic::AtomicUsize;
use yew::{prelude::*, suspense::Suspension};
use yew_router::{history::AnyHistory, prelude::*};

pub static PAGE_RENDER_COUNT: AtomicUsize = AtomicUsize::new(0);

#[function_component(ClientApp)]
pub fn csr_app_component() -> Html {
    use crate::services::state::get_initial_ssr_state;

    let route = use_state_eq(|| None);
    let route_callback = {
        let route = route.clone();
        Callback::from(move |r| route.set(Some(r)))
    };

    html! {
        <BounceRoot>
            <InitialPageStateSetter
                page_state={PageState::new(None)}
                ssr_state={get_initial_ssr_state()}
                route={(*route).clone()}
            />
            <Suspense>
                <AppRoute on_render_route={route_callback.clone()} history={None} />
            </Suspense>
        </BounceRoot>
    }
}
#[derive(PartialEq, Properties)]
struct AppRouteProps {
    on_render_route: Option<Callback<Route>>,
    history: Option<AnyHistory>,
}
#[function_component(AppRoute)]
fn app_route(props: &AppRouteProps) -> HtmlResult {
    let (suspension, _handle) = Suspension::new();

    let page_state = use_atom::<PageState>();

    if page_state.route.is_none() {
        Err(RenderError::Suspended(suspension))?;
    }

    Ok(if cfg!(target_arch = "wasm32") {
        html! {
            <BrowserRouter>
                <Switch<Route> render={{
                    let on_render_route = props.on_render_route.clone().unwrap();
                    move |r: Route| {
                        on_render_route.emit(r.clone());
                        switch(r)
                    }
                }}/>
            </BrowserRouter>
        }
    } else {
        html! {
            <Router history={props.history.clone().unwrap()}>
                <Switch<Route> render={switch} />
            </Router>
        }
    })
}

#[derive(Properties, PartialEq, Default)]
pub struct ServerAppProps {
    pub state: PageState,
    pub ssr_state: SSRState,
}
#[function_component(ServerApp)]
pub fn ssr_app_component(props: &ServerAppProps) -> Html {
    use yew_router::history::History as _;

    let history = yew_router::history::AnyHistory::from(yew_router::history::MemoryHistory::new());
    let _ = history.push_with_query(
        props.state.route.clone().unwrap_or_default(),
        &std::collections::HashMap::<String, String>::new(),
    );

    html! {
        <BounceRoot>
            <InitialPageStateSetter
                page_state={props.state.clone()}
                ssr_state={props.ssr_state.clone()}
                route={None}
            />
            <Suspense>
                <AppRoute on_render_route={Option::<Callback<Route>>::None} history={Some(history)} />
            </Suspense>
        </BounceRoot>
    }
}

#[derive(PartialEq, Properties)]
struct PageStateSetterProps {
    pub page_state: PageState,
    pub ssr_state: SSRState,
    pub route: Option<Route>,
}
#[function_component(InitialPageStateSetter)]
fn initial_page_state_setter_component(props: &PageStateSetterProps) -> Html {
    let page_state = use_atom::<PageState>();
    let config = use_atom::<Configuration>();
    let counter = use_atom::<Counter>();
    // The Default implementation for [`PageState`] is to set `initialized` to false.
    // Because this runs every time a new page is loaded, we want to only update the page state
    // to the *initial* value if it's not already initialized.
    if !page_state.initialized {
        let mut state = props.page_state.clone();
        state.route = Some(props.ssr_state.route.clone());
        page_state.set(state);
        config.set(props.ssr_state.config.clone());
        counter.set(Counter::new(props.ssr_state.count));
    } else if cfg!(target_arch = "wasm32") {
        crate::services::state::save_config(&config);
        crate::services::state::update_config_html_class(&config)
    }
    if page_state.yew_route != props.route {
        if let Some(ref route) = props.route {
            debug!("changing route to {route}");
        }
        page_state.set(PageState {
            yew_route: props.route.clone(),
            ..(*page_state).clone()
        });
    }
    html! {
         <span></span>
    }
}
