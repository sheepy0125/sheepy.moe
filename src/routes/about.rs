//! A more personal "about" page

use crate::{components::main_page_content::MainPageContent, router::Route};

use yew::prelude::*;

#[function_component(About)]
pub fn about_component() -> Html {
    html! {
        <MainPageContent page={Route::Homepage} first_widget={html! {
            <section id="about-information" class="widget">
                <div class="widget-banner">
                    <h1 class="section-header">{"[more] about me!"}</h1>
                    <span class="w-full flex-1" />  // spacer
                    <h3 class="subtle-emphasis-text hidden md:block h-max">
                        <span>{"what do "}</span>
                        <em class="italic">{"you"}</em>
                        <span>{" care? /lh"}</span>
                    </h3>
                </div>
                <div id="about-information-content">
                    <p class="font-bold">{"I'm a minor."}</p>
                    <p>
                        {"My name is "}
                        <span class="subtle-emphasis-text">
                            <span class="underline">{"Ryan"}</span>
                            {" "}
                            <span class="underline">{"Sylvia"}</span>
                            {" "}
                            <span class="underline">{"[redacted]"}</span>
                        </span>
                        {"! "}
                    </p>
                    <p>
                        {"I enjoy computers, talking with folks, niche internet culture, problem-solving, and programming."}
                    </p>
                </div>
            </section>
        }}>
            <></>
        </MainPageContent>
    }
}
