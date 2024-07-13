use crate::{
    components::main_page_content::MainPageContent,
    router::{BlogRoute, Route},
};

use async_trait::async_trait;
use bounce::{
    query::{use_prepared_query, Query, QueryResult},
    BounceStates,
};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use std::rc::Rc;
use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, JsCast as _};
#[cfg(target_arch = "wasm32")]
use web_sys::Element;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn parse_blog_html(_node: NodeRef, _callback: Callback<BlogRoute>) {}
/// Parse the HTML returned from a blog, changing inter-website `a` tags to `Link` tags for SPA re-rendering
#[cfg(target_arch = "wasm32")]
fn parse_blog_html(node: NodeRef, callback: Callback<BlogRoute>) {
    // Update a tag that is guaranteed to have a `href`
    fn update_a_tag(element: &Element, callback: Callback<BlogRoute>) {
        let callback = callback.clone();
        let href = element
            .get_attribute("href")
            .expect("link didn't have href");

        let link = match href.split_once("/blog/") {
            Some((_, page)) => BlogRoute::BlogFullPath {
                full_path: page.to_string(),
            },
            // Not a blog link
            None => return,
        };

        let closure = Closure::<dyn FnMut(_)>::new({
            let callback = callback.clone();
            move |e: MouseEvent| {
                if e.meta_key() || e.ctrl_key() || e.shift_key() || e.alt_key() {
                    return;
                }

                callback.clone().emit(link.clone());
                e.prevent_default();
            }
        });
        element
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        // funky memory leak
        closure.forget();
    }

    /// Search through all children and update links
    fn dfs_update_links(element: &Element, callback: Callback<BlogRoute>) {
        if element.has_attribute("href") {
            update_a_tag(element, callback.clone());
            return;
        }

        // [`HtmlCollection`] doesn't implement Iter, so we must use [`HtmlCollection::get`] on each index
        let (children, children_len) = (element.children(), element.child_element_count());
        for index in 0..children_len {
            dfs_update_links(
                &children
                    .get_with_index(index)
                    .expect("children of node changed length"),
                callback.clone(),
            );
        }
    }

    let element = node.cast().expect("blog node ref yet to be rendered");
    dfs_update_links(&element, callback);
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[error("Failed to fetch blog post!")]
struct BlogQueryError;

/// A blog query fetched from the backend
#[derive(Debug, PartialEq, Clone)]
struct BlogQuery {
    /// [`IString`] doesn't impl Serialize nor Deserialize, so we have intermediary custom impls
    /// to convert it to a &str
    post: AttrValue,
}
impl Serialize for BlogQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("BlogQuery", 1)?;
        s.serialize_field("post", self.post.as_str())?;
        s.end()
    }
}
impl<'de> Deserialize<'de> for BlogQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self { post: s.into() })
    }
}
#[async_trait(?Send)]
impl Query for BlogQuery {
    type Input = String;
    type Error = BlogQueryError;

    async fn query(_states: &BounceStates, input: Rc<Self::Input>) -> QueryResult<Self> {
        let post;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let port = dotenvy::var("PORT").unwrap();
            let port = port.parse::<u16>().unwrap();
            let resp = reqwest::get(&format!("http://127.0.0.1:{port}/raw_blog/{input}"))
                .await
                .map_err(|_| BlogQueryError)?;
            post = resp.text().await.map_err(|_| BlogQueryError)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            let request = gloo_net::http::Request::get(&format!("/raw_blog/{input}"))
                .build()
                .map_err(|_| BlogQueryError)?;
            let resp = request.send().await.map_err(|_| BlogQueryError)?;
            post = resp.text().await.map_err(|_| BlogQueryError)?;
        }
        Ok(Rc::new(Self { post: post.into() }))
    }
}

#[derive(PartialEq, Properties)]
pub struct BlogPostProps {
    pub post: AttrValue,
    pub ready_callback: Callback<NodeRef>,
}
#[function_component(BlogPost)]
pub fn blog_post_component(props: &BlogPostProps) -> HtmlResult {
    let blog_post_node = use_node_ref();
    // During a forced SPA refresh, the component won't be reloaded. We use this state to
    // manually invalidate the blog query.
    let blog_post_state = use_state_eq(|| props.post.clone());

    let post = Rc::new(props.post.clone());
    let post = use_prepared_query::<BlogQuery>(post.to_string().into())?;

    // Manually invalidate query if the post state changed from the props (component wasn't reloaded).
    // FIXME: Why does the query hang in a suspense?
    if *blog_post_state != props.post {
        blog_post_state.set(props.post.clone());
        let post = post.clone();
        spawn_local(async move {
            let _ = post.refresh().await;
        })
    }

    props.ready_callback.emit(blog_post_node.clone());

    Ok(match &*post {
        Err(_) => html! {{"Failed to fetch post!"}},
        Ok(ref post) => html! {
            <div ref={blog_post_node}>
                {Html::from_html_unchecked(AttrValue::clone(&post.post))}
            </div>
        },
    })
}

#[derive(PartialEq, Properties)]
pub struct BlogProps {
    pub post: AttrValue,
}
#[function_component(Blog)]
pub fn blog_component(props: &BlogProps) -> Html {
    let navigator = use_navigator().expect("failed to get navigator");

    // Override the default /blog/* link click behavior
    let link_click_callback = {
        let navigator = navigator.clone();
        Callback::from(move |link: BlogRoute| navigator.push(&link))
    };
    let blog_node_ready_callback = {
        let link_click_callback = link_click_callback.clone();
        Callback::from(move |blog_node: NodeRef| {
            let link_click_callback = link_click_callback.clone();
            spawn_local(async move {
                // fixme: we need to wait until the blog is rendered before we can parse
                // its html. but we can't use a state because force-pushing to the navigator
                // doesn't trigger a "brand new" component to render, instead keeping the
                // state even though the props changed. also, sleep doesn't even work -- this
                // is the stupidest hack i've ever done in yew.rs wtf
                #[cfg(target_arch = "wasm32")]
                wasmtimer::tokio::sleep(std::time::Duration::from_secs(1));
                parse_blog_html(blog_node.clone(), link_click_callback.clone());
            });
        })
    };

    html! {
         <MainPageContent page={Route::BlogRoot} first_widget={html! {
            <section class="widget">
                <Suspense fallback={html!{"Loading post!"}}>
                    <div class="markdown">
                        <BlogPost post={props.post.clone()} ready_callback={blog_node_ready_callback} />
                    </div>
                </Suspense>
            </section>
        }}>
            <div class="mx-auto w-max flex gap-4">
                <Link<Route> to={Route::BlogRoot} classes="link block w-max">{"back to all blogs"}</Link<Route>>
                <a href={format!("/raw_blog/{}", &props.post)} class="link block w-max mx-auto">{"open original"}</a>
            </div>
        </MainPageContent>
    }
}
