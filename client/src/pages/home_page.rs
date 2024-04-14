use yew::prelude::*;
use yewdux::prelude::use_store;
use crate::components::{song_card::SongCard};
use crate::store::Store;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    // Get the search results from the store
    let (store, _) = use_store::<Store>();
    let search_results = store.search_results.clone();

    html! {
        <section class="h-full">
            <div>
                <h3 class="my-0 mb-2">{"Similar Songs"}</h3>
                <small>{"Based on your search"}</small>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 auto-cols-auto">
                // Loop through results and display a SongCard for each
                {for search_results.iter().map(|song| {
                    html! {
                        <SongCard song={song.clone()} />
                    }
                })}
            </div>
        </section>
    }
}