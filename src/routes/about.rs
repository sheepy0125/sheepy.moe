//! A more personal "about" page

use crate::{
    components::main_page_content::MainPageContent,
    router::{BlogRoute, Route},
};

use yew::prelude::*;
use yew_router::prelude::*;

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
                        {"I really enjoy computers (includes programming and what not!). "}
                        {"I also think Niche Internet Communities also are quite poggers (e.g. fedi, communities for my hyperfixations, "}
                        {"networks of personal websites, etc.). "}
                    </p>
                    <p>
                        {"I'd like to think of myself in terms of positive personality traits! "}
                        {"I've (mostly) tried my best to be kind and honest when interacting with others. "}
                        {"I tend to be optimistic and easygoing while also serious, if that makes sense? "}
                        {"In any event, I'm not "}<span class="italic">{"really"}</span>{" mature, yet."}
                    </p>
                    <p>
                        {"I like words a lot. You can check out "}
                        <Link<BlogRoute> to={BlogRoute::BlogFullPath { full_path: "words".to_string() }}>{"/blog/words"}</Link<BlogRoute>>
                        {" for a list of some cool ones."}
                    </p>
                    <p>
                        {"As for labels, I am (non-exhaustive): "}
                        {"{person, girl, linux enthusiast, rustacean, programmer, cute enby transfem, autistic, minor}"}
                    </p>
                    <p>{"I'm really not sure what else to put here. Maybe you could check out my blog posts while you're here? Love you! <3 /p"}</p>
                </div>
            </section>
        }}>
            <></>
        </MainPageContent>
    }
}
