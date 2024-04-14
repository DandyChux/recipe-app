use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Callback, Component, Html, NodeRef, Properties, Context};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::{spawn_local};
use crate::store::{Store, set_search_input, set_search_results, clear_search_results, set_show_alert};
use crate::api::search_api::api_search_songs;
use super::input::Input;
use crate::router::Route;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SearchProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub onsearch: Callback<()>,
}

#[function_component(Search)]
pub fn search(props: &SearchProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();
    let search_input = store.search_input.clone();
    let search_input_ref = NodeRef::default();

    let handle_search = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();
        println!("searching");

        Callback::from(move |_: MouseEvent| {
            let dispatch = store_dispatch.clone();
            let navigator = cloned_navigator.clone();
            let search_input = search_input.clone();
            spawn_local(async move {
                // Call the search API
                let result = api_search_songs(search_input).await;
                match result {
                    Ok(songs) => {
                        set_search_results(songs, dispatch.clone());
                        // navigator.push(&Route::ProfilePage);
                    }
                    Err(e) => {
                        set_search_results(vec![], dispatch.clone());
                        set_show_alert(e.message.to_string(), dispatch.clone());
                    }
                };
            });
        })
    };

    let handle_input = {
        let store_dispatch = dispatch.clone();

        Callback::from(move |value: String| {
            let dispatch = store_dispatch.clone();
            dispatch.reduce_mut(move |store| {
                store.search_input = value;
                store.search_results.clear();
            });
        })
    };

    let handle_on_input_blur = {
        let store_dispatch = dispatch.clone();

        Callback::from(move |(name, value): (String, String)| {
            let dispatch = store_dispatch.clone();
            dispatch.reduce_mut(move |store| {
                store.search_input = value;
                store.search_results.clear();
            });
        })
    };

    let class = classes!(
        "bg-input",
        props.class.clone()
    );

    html! {
        <Input
            class={class.to_string()}
            name="search"
            input_type="search"
            input_ref={search_input_ref.clone()}
            placeholder={props.placeholder.clone()}
            handle_on_input_blur={handle_on_input_blur}
            handle_onchange={handle_input}
            onsearch={handle_search}
        />
    }
}