use crate::{
    router::{self, Route}, 
    store::{set_loading, set_show_alert, Store, set_auth_user},
    api::user_api::{api_logout_user}
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub class: String,
}

#[function_component(Header)]
pub fn header_component(props: &HeaderProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    let handle_logout = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let dispatch = store_dispatch.clone();
            let navigator = cloned_navigator.clone();
            spawn_local(async move {
                set_loading(true, dispatch.clone());
                let result = api_logout_user().await;
                match result {
                    Ok(_) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert("Logout successful".to_string(), dispatch.clone());
                        set_auth_user(None, dispatch.clone());
                        navigator.push(&router::Route::LoginPage);
                    }
                    Err(e) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch.clone());
                    }
                };
            });
        })
    };

    let class = classes!(
        "h-20",
        "bg-background",
        props.class.clone()
    );

    html! {
        <header class={class}>
            <nav class="container flex items-center justify-between h-full">
                <div>
                    <Link<Route> to={Route::HomePage} classes="font-bold">{"Rusty Melody"}</Link<Route>>
                </div>
                
                <ul class="flex items-center gap-6">
                    <li>
                        <Link<Route> to={Route::HomePage} classes="font-bold">{"Home"}</Link<Route>>
                    </li>
                    if user.is_some() {
                        <>
                            <li>
                                <Link<Route> to={Route::ProfilePage} classes="font-bold">{"Profile"}</Link<Route>>
                            </li>
                            <li class="cursor-pointer" onclick={handle_logout}>
                                {"Logout"}
                            </li>
                        </>

                    } else {
                        <>
                            <li>
                                <Link<Route> to={Route::RegisterPage} classes="font-bold">{"SignUp"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::LoginPage} classes="font-bold">{"Login"}</Link<Route>>
                            </li>
                        </>
                    }
                </ul>
            </nav>
        </header>
    }
}