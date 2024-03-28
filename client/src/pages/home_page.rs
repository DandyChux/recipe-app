use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <section class="h-full pt-20">
            <div class="max-w-4xl mx-auto rounded-md h-[20rem] flex justify-center items-center">
                <p class="text-3xl font-semibold">{"Welcome to Rusty Melody"}</p>
            </div>
        </section>
    }
}