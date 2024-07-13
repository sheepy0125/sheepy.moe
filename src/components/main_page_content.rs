use crate::{
    components::{counter::Counter, navbar::Navbar, theme_toggle::ThemeToggle},
    router::Route,
    types::{Configuration, PageState},
};

use bounce::use_atom;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MainPageContentProps {
    pub page: Route,
    pub first_widget: Html,
    pub children: Html,
}

#[function_component(MainPageContent)]
pub fn main_page_content_component(props: &MainPageContentProps) -> Html {
    let config = use_atom::<Configuration>();
    let page_state = use_atom::<PageState>();

    html! {
         <div class="w-full">
            if config.show_banner
                || page_state.yew_route == Some(Route::Homepage)
                || (page_state.yew_route.is_none() && page_state.route == Some("/home".to_string()))
            {
                <header id="banner">
                    <div class="w-max mx-auto">
                        <h1 class="text-3xl font-bold underline-offset-2">
                            <span class="subtle-underline">{"salutations"}</span>
                            <span>{", i'm "}</span>
                            <span class="emphasis-text">{"sheepy"}</span>
                            <span>{"!"}</span>
                        </h1>
                        <h2 class="text-lg text-right subtle-text font-medium">
                            <span class="italic">{"also known as "}</span>
                            <span class="subtle-emphasis-text">{"sheepy0125"}</span>
                            <span class="italic">{" online"}</span>
                        </h2>
                    </div>
                </header>
            }
            <div class="p-4 split-content">
                <div class="mobile-adjoined-header-asides">
                    <aside class="nav-and-adjoined-aside">
                        <Navbar current_page={props.page.clone()} />
                        <ThemeToggle />
                    </aside>
                    <aside class="w-0 md:w-max">
                        <Counter />
                    </aside>
                </div>
                {props.first_widget.clone()}
            </div>
            {props.children.clone()}
         </div>
    }
}
