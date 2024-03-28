use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlSelectElement, Event};
use yew::prelude::*;
use common::schema::select::SelectItem;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectProps {
    /// The `label` attribute for the input.
    pub label: String,
    /// The `name` attribute for the input.
    pub name: String,
    /// The controlled value of this form element
    pub value: String,
    /// The `input_ref` attribute for the input.
    pub input_ref: NodeRef,
    /// The callback to be used for the `onchange` event.
    pub handle_onchange: Callback<String>,
    /// The callback to be used for the `onblur` event.
    pub handle_on_input_blur: Callback<(String, String)>,
    /// The `ValidationErrors` for the form.
    pub errors: Rc<RefCell<ValidationErrors>>,
    /// The `option` & `optgroup` elements to be rendered within the select element.
    #[prop_or_default]
    pub items: Vec<SelectItem>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub multiple: bool,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
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

    let class = classes!(
        "block",
        "w-full",
        "py-2",
        "px-4",
        "border",
        "rounded-lg",
        // "appearance-none",
        "border-1",
        "focus:outline-none",
        props.classes.clone(),
    );

    let handle_onchange = props.handle_onchange.reform(|event: Event| {
        let select: HtmlSelectElement = event.target_dyn_into().expect_throw("event target should be a select element");
        let selected_values: Vec<String> = if select.multiple() {
            let selected_options = select.selected_options();
            let mut values = vec![];
            for i in 0..selected_options.length() {
                let option = selected_options.get_with_index(i).expect_throw("should get option");
                values.push(option.inner_html());
            }
            values
        } else {
            vec![select.value()]
        };
        selected_values.join(",")
    });

    let handle_on_input_blur = props.handle_on_input_blur.clone();
    let on_blur = {
        let cloned_input_name = props.name.clone();
        Callback::from(move |event: FocusEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlSelectElement>().value();

            handle_on_input_blur.emit((cloned_input_name.clone(), value));
        })
    };

    html! {
        <div>
            if !props.label.is_empty() {
                <label 
                    class="block mb-2 text-sm font-semibold text-foreground/80" 
                    for={props.name.clone()}
                >
                    {props.label.clone()}
                </label>
            }

            <select
                class={class}
                name={props.name.clone()}
                ref={props.input_ref.clone()}
                onchange={handle_onchange}
                onblur={on_blur}
                value={props.value.clone()}
                disabled={props.disabled}
                multiple={props.multiple}
            >
                { for props.items.iter().map(|item| {
                    html! {
                        <option class="bg-input" value={item.value.clone()}>{item.label.clone()}</option>
                    }
                }) }
            </select>

            <span class="block pt-1 text-xs text-destructive">
                {error_message}
            </span>
        </div>
    }
}