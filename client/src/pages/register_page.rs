use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user_api::api_register_user;
use crate::components::ui::{input::Input, button::Button, select::Select};
use crate::router::{self, Route};
use crate::store::{set_loading, set_show_alert, Store};
use common::schema::platform::get_platform_select_items;
use common::schema::user::SignupUserSchema;

use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<SignupUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "name" => data.name = value,
            "email" => data.email = value,
            "preferred_platform" => data.preferred_platform = Some(value),
            "password" => data.password = value,
            "password_confirm" => data.password_confirm = value,
            "username" => data.username = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form = use_state(|| SignupUserSchema::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let name_input_ref = NodeRef::default();
    let email_input_ref = NodeRef::default();
    let preferred_platform_input_ref = NodeRef::default();
    let username_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let password_confirm_input_ref = NodeRef::default();

    let platform_select_items = get_platform_select_items();

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "email" => data.email = value,
                "password" => data.password = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_name_input = get_input_callback("name", form.clone());
    let handle_email_input = get_input_callback("email", form.clone());
    let handle_username_input = get_input_callback("username", form.clone());
    let handle_preferred_platform_input = get_input_callback("preferred_platform", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());
    let handle_password_confirm_input = get_input_callback("password_confirm", form.clone());

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        let cloned_navigator = navigator.clone();
        let cloned_dispatch = dispatch.clone();

        let cloned_name_input_ref = name_input_ref.clone();
        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_username_input_ref = username_input_ref.clone();
        let cloned_preferred_platform_input_ref = preferred_platform_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();
        let cloned_password_confirm_input_ref = password_confirm_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();
            let dispatch = cloned_dispatch.clone();

            let name_input_ref = cloned_name_input_ref.clone();
            let email_input_ref = cloned_email_input_ref.clone();
            let username_input_ref = cloned_username_input_ref.clone();
            let preferred_platform_input_ref = cloned_preferred_platform_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();
            let password_confirm_input_ref = cloned_password_confirm_input_ref.clone();

            event.prevent_default();
            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        let form_json = serde_json::to_string(&form_data).unwrap();
                        set_loading(true, dispatch.clone());

                        let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let username_input = username_input_ref.cast::<HtmlInputElement>().unwrap();
                        let preferred_platform_input = preferred_platform_input_ref
                            .cast::<HtmlSelectElement>()
                            .unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_confirm_input = password_confirm_input_ref
                            .cast::<HtmlInputElement>()
                            .unwrap();

                        name_input.set_value("");
                        email_input.set_value("");
                        username_input.set_value("");
                        preferred_platform_input.set_value("");
                        password_input.set_value("");
                        password_confirm_input.set_value("");

                        let res = api_register_user(&form_json).await;
                        match res {
                            Ok(_) => {
                                set_loading(false, dispatch.clone());
                                set_show_alert(
                                    "Account registered successfully".to_string(),
                                    dispatch,
                                );
                                navigator.push(&router::Route::LoginPage);
                            }
                            Err(e) => {
                                set_loading(false, dispatch.clone());
                                set_show_alert(e.to_string(), dispatch);
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
        <section class="grid h-full py-8 place-items-center">
            <div class="w-full">
                <h1 class="text-4xl xl:text-6xl text-center font-[600] text-primary mb-4">
                    {" Welcome to Rusty Melody!"}
                </h1>
                <h2 class="mb-4 text-lg text-center">
                    {"Sign Up To Get Started!"}
                </h2>
                
                <form
                    onsubmit={on_submit}
                    class="w-full max-w-md p-8 mx-auto space-y-5 overflow-hidden shadow-lg rounded-2xl"
                >
                    <Input 
                        label="Full Name" 
                        name="name" 
                        input_ref={name_input_ref} 
                        handle_onchange={handle_name_input}  
                        errors={&*validation_errors} 
                        handle_on_input_blur={validate_input_on_blur.clone()} 
                    />
                    <Input 
                        label="Username" 
                        name="username" 
                        input_ref={username_input_ref} 
                        handle_onchange={handle_username_input}  
                        errors={&*validation_errors} 
                        handle_on_input_blur={validate_input_on_blur.clone()} 
                    />
                    <Input 
                        label="Email" 
                        name="email" 
                        input_type="email" 
                        input_ref={email_input_ref} 
                        handle_onchange={handle_email_input} 
                        errors={&*validation_errors} 
                        handle_on_input_blur={validate_input_on_blur.clone()} 
                    />
                    <Select 
                        label="Preferred Platform"
                        name="preferred_platform"
                        input_ref={preferred_platform_input_ref}
                        handle_onchange={handle_preferred_platform_input}
                        errors={&*validation_errors}
                        handle_on_input_blur={validate_input_on_blur.clone()}
                        value={form.deref().preferred_platform.clone().unwrap_or_default()}
                        items={platform_select_items}
                    />
                    <Input 
                        label="Password" 
                        name="password" 
                        input_type="password" 
                        input_ref={password_input_ref} 
                        handle_onchange={handle_password_input} 
                        errors={&*validation_errors} 
                        handle_on_input_blur={validate_input_on_blur.clone()} 
                    />
                    <Input
                        label="Confirm Password"
                        name="password_confirm"
                        input_type="password"
                        input_ref={password_confirm_input_ref}
                        handle_onchange={handle_password_confirm_input}
                        errors={&*validation_errors}
                        handle_on_input_blur={validate_input_on_blur.clone()}
                    />
                    
                    <span class="block">
                        {"Already have an account?"} {" "}
                    <Link<Route> to={Route::LoginPage} classes="text-info">{"Login Here"}</Link<Route>>
                    </span>

                    <Button
                        loading={store.loading}
                        btn_type={"submit"}
                    >
                        {" Sign Up"}
                    </Button>
                </form>
            </div>
        </section>
    }
}