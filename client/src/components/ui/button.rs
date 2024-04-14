use super::spinner::Spinner;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or("button".to_string())]
    pub btn_type: String,
    #[prop_or("bg-primary".to_string())]
    pub btn_color: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or("text-primary-foreground".to_string())]
    pub text_color: String,
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Button)]
pub fn button_component(props: &ButtonProps) -> Html {
    let btn_type = props.btn_type.clone();
    let btn_color = props.btn_color.clone();
    let text_color = props.text_color.clone();
    let disabled = props.disabled;
    let loading = props.loading;
    let onclick = props.onclick.clone();
    
    let class = classes!(
        if disabled { "cursor-not-allowed" } else { "hover:bg-primary/60 hover:text-primary-foreground" },
        if loading {"bg-[#ccc]".to_string()} else {btn_color.clone()},
        // "py-4",
        // "px-8",
        "transition-all",
        "duration-500",
        "border",
        "border-black",
        "rounded-md",
        &text_color,
        &props.class,
    );

    html! {
        <button
            type={btn_type}
            class={class}
            {disabled}
            {onclick}
        >
            if loading {
                <div class="flex items-center gap-2">
                    <Spinner />
                    <span class="inline-block text-muted">{"Loading..."}</span>
                </div>
            } else {
                <span class={text_color.to_owned()}>{props.children.clone()}</span>
            }
        </button>
    }
}