use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use super::button::Button;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    /// Input type (text, password, email, etc)
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or_default]
    /// Input label
    pub label: String,
    /// Input name
    #[prop_or("".to_string())]
    pub name: String,
    /// Reference to the input element
    #[prop_or_default]
    pub input_ref: NodeRef,
    /// Callback to handle the input change event
    pub handle_onchange: Callback<String>,
    #[prop_or_default]
    /// Callback to handle the input blur event
    pub handle_on_input_blur: Callback<(String, String)>,
    #[prop_or_default]
    /// Validation errors
    pub errors: Rc<RefCell<ValidationErrors>>,
    /// Input value
    #[prop_or_default]
    pub value: String,
    /// Input placeholder
    #[prop_or("".to_string())]
    pub placeholder: String,
    /// Input class
    #[prop_or_default]
    pub class: String,
    /// Callback to handle search event
    #[prop_or_default]
    pub onsearch: Callback<MouseEvent>,
}

#[function_component(Input)]
pub fn input_component(props: &InputProps) -> Html {
    let input_type = props.input_type.clone();
    let value = props.value.clone();
    let placeholder = props.placeholder.clone();
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

    let class = classes!(
        "block",
        "w-full",
        if input_type == "search" { "px-6 pl-8 pr-[6rem]" } else { "px-4" },
        if input_type == "search" { "py-4" } else { "py-2" },
        "border",
        "rounded-sm",
        "appearance-none",
        "border-1",
        "focus:outline-none",
        props.class.clone()
    );

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

            <div class="relative flex items-center">    
                if props.input_type == "search" {
                    <span class="absolute top-4 left-2">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
                        </svg>                          
                    </span>
                }

                <input 
                    type={input_type}
                    placeholder={placeholder}
                    class={class}
                    ref={props.input_ref.clone()}
                    onchange={onchange}
                    onblur={on_blur}
                    name={props.name.clone()}
                />

                if props.input_type == "search" {
                    <Button 
                        class="absolute px-4 py-2 right-2"
                        onclick={props.onsearch.clone()}
                    >
                        {"Search"}
                    </Button>
                }
            </div>

            <span class="block pt-1 text-xs text-destructive">
                {error_message}
            </span>
        </div>
    }
}