//! Navbar component

use crate::router::{Route, ITER_ROUTES};

use yew::prelude::*;
use yew_router::components::Link;

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    pub current_page: Route,
}

#[function_component(Navbar)]
pub fn navbar_component(props: &NavbarProps) -> Html {
    let current_page = &props.current_page;

    html! {
        <nav>
            <ul class="nav-button-holder">
            {
                ITER_ROUTES.iter().map(|route| html!{
                    <div class="not-link">
                        <Link<Route> to={route.clone()}>
                            <button selected={route == current_page} class="nav-button">
                                {route.to_string()}
                            </button>
                        </Link<Route>>
                    </div>
               }).collect::<Html>()
           }
            </ul>
        </nav>
   }
}
