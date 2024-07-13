//! Homepage

use crate::{
    components::main_page_content::MainPageContent,
    constants::BUTTON_88X31S,
    links::{EXTERNAL_EMAIL, EXTERNAL_MASTODON, EXTERNAL_MATRIX, EXTERNAL_PRONOUNS},
    router::{BlogRoute, Route},
};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Homepage)]
pub fn homepage_component() -> Html {
    html! {
        <MainPageContent page={Route::Homepage} first_widget={html!{
            <section id="generic-about-information" class="widget">
                <div class="widget-banner">
                    <h1 class="section-header">{"generic about me!"}</h1>
                    <span class="w-full flex-1" />  // spacer
                    <aside class="pill-holder flex-none">
                        <div class="pill group/pill">
                            <span class="pill-label">{"name"}</span>
                            <span class="pill-value group-hover/pill:bg-light-blahaj-250 dark:group-hover/pill:bg-dark-blahaj-150">{"ryan"}</span>
                        </div>
                        <div class="pill group/pill">
                            <span class="pill-label"><a href={EXTERNAL_PRONOUNS} class="underline">{"pronouns"}</a></span>
                            <span class="pill-value group-hover/pill:bg-light-blahaj-250 dark:group-hover/pill:bg-dark-blahaj-150">{"she/it"}</span>
                        </div>
                    </aside>
                </div>
                <main>
                    <p>{"Thanks for stopping by my personal website!! "}</p>
                    <p>
                        <span>{"I'm a hobbyist programmer who is mostly interested in embedded systems and electrical engineering. "}</span>
                        <span>{"You can see some of my programming projects at "}</span>
                        <Link<BlogRoute> to={BlogRoute::BlogFullPath {full_path: "projects/index".into()}}>{"/blog/projects/index"}</Link<BlogRoute>>
                        <span>{"."}</span>
                    </p>

                    <p>
                        {"If you wanna learn more about me, check out "}
                        <Link<Route> to={crate::router::Route::About}>{"/about"}</Link<Route>>
                        <span>{"."}</span>
                    </p>
                </main>
            </section>
        }}>
            <section class="buttons-88x31">
                <label for="buttons-88x31" class="mx-auto w-max block">
                    <p class="dark:text-white">{"some badges. please don't hotlink!"}</p>
                </label>
                <div id="buttons-88x31">
                    {BUTTON_88X31S.iter().map(|button| {
                        let (alt, src, href) = (button.alt.to_string(), button.src.to_string(), button.href.to_string());
                            html! {
                                <stamp>
                                    <a href={href.clone()} target={if !href.starts_with('#') {Some("_blank")} else {None}}>
                                        <img alt={alt.clone()} title={alt} src={format!("static/88x31/{src}")} />
                                    </a>
                                </stamp>
                        }
                }).collect::<Html>()}
                </div>
            </section>
            <div class="split-content">
                <section id="contact-information" class="widget">
                    <div class="widget-banner">
                        <h1 class="section-header">{"contact me!"}</h1>
                        <span class="w-full flex-1" />  // spacer
                        <h3 class="subtle-emphasis-text hidden md:block h-max">{"please, i'm lonely /j"}</h3>
                    </div>
                    <div id="contact-information-content">
                        <main>
                            <p>
                            {"Hey, please don't feel afraid to contact me! "}
                            <span class="font-bold">{"Please keep in mind that I am a minor."}</span>
                            </p>
                            <p>
                            {"I do really enjoy speaking to random people on this World Wide Web! "}
                            {"If you share an interest with me or were "}
                            <span class="italic">{"wowed"}</span>
                            {" by something I did, I'd be more than happy to discuss it!"}
                            </p>
                            <table class="list-odd-even">
                                <tbody>
                                    <tr>
                                        <td class="font-semibold text-center">{"Discord "}</td>
                                        <td class="subtle-emphasis-text">{"sheepy0125"}</td>
                                    </tr>
                                    <tr>
                                        <td class="font-semibold text-center">{"Email "}</td>
                                        <td class="subtle-emphasis-text">
                                            <span class="subtle-underline">{EXTERNAL_EMAIL}</span>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="font-semibold text-center">{"Mastodon  "}</td> // hacky fix for weird spacing
                                        <td class="subtle-emphasis-text">
                                            <a href={EXTERNAL_MASTODON} class="subtle-underline">{"@sheepy0125@wetdry.world"}</a>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="font-semibold text-center">{"Matrix "}</td>
                                        <td class="subtle-emphasis-text">{EXTERNAL_MATRIX}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </main>
                    </div>
                </section>
            </div>
        </MainPageContent>
    }
}
