use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <section class="h-full">
            <div>
                <h3 class="my-0 mb-2">{"Similar Songs"}</h3>
                <small>{"Based on your search"}</small>
            </div>

            <div class="grid grid-cols-2 auto-cols-auto">
                // Loop through results and display a SongCard for each
            </div>
        </section>
    }
}