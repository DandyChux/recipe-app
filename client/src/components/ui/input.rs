use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or("text".to_string())]
    pub input_type: String,
    pub label: String,
    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component(Input)]
pub fn input_component(props: &InputProps) -> Html {
    let input_type = props.input_type.clone();
    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get(&props.name.as_str()) {
        Some(err) => err,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();

        handle_onchange.emit(value);
    });

    let handle_on_input_blur = props.handle_on_input_blur.clone();
    let on_blur = {
        let cloned_input_name = props.name.clone();
        Callback::from(move |event: FocusEvent| {
            let input_name = cloned_input_name.clone();
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();

            handle_on_input_blur.emit((input_name.clone(), value));
        })
    };

    html! {
        <div>
            if !props.label.is_empty() {
                <label
                    html={props.name.clone()}
                    class={classes!(
                        "block", "mb-2",
                        "font-semibold",
                        "sm:text-sm", "text-foreground/80"
                    )}
                >
                    {props.label.clone()}
                </label>
            }

            <input 
                type={input_type}
                placeholder=""
                class="block w-full px-4 py-2 border rounded-lg appearance-none border-1 focus:outline-none"
                ref={props.input_ref.clone()}
                onchange={onchange}
                onblur={on_blur}
            />

            <span class="block pt-1 text-xs text-destructive">
                {error_message}
            </span>
        </div>
    }
}