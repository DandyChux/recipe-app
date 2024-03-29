use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <section class="h-full">
        <div>
            <h3 class="my-0">{"Similar Songs"}</h3>
            <small>{"Based on your search"}</small>
        </div>
        </section>
    }
}