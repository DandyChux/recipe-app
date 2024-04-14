use crate::{
    router::{self, Route}, 
    store::{set_loading, set_show_alert, Store},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use common::schema::song::Song as CommonSong;

#[derive(Clone, Properties, PartialEq)]
pub struct SongProps {
    pub song: CommonSong,

}

#[function_component(SongCard)]
pub fn song_card(props: &SongProps) -> Html {
    let song = props.song.clone();

    let handle_play = {
        // let cloned_navigator = navigator.clone();
        // let cloned_song = song.clone();

        Callback::from(move |_: MouseEvent| {
            let audio = web_sys::HtmlAudioElement::new_with_src(song.url.as_str()).unwrap();
            spawn_local(async move {
                audio.load();
                let _ = audio.play().unwrap();
            });
        })
    };

    html! {
        <div class="flex items-center flex-1 p-2 m-2 rounded-md shadow-md bg-background">
            <img src={song.cover.clone()} class="w-32 h-32 rounded-md" />
            <div class="flex flex-col items-center mt-2">
                <h3 class="text-lg font-semibold">{&song.title}</h3>
                <p class="text-sm text-secondary">{&song.artist}</p>
                <button
                    class="px-4 py-1 mt-2 text-sm font-semibold rounded-md bg-primary"
                    onclick={handle_play}
                >
                    {"Play"}
                </button>
            </div>
        </div>
    }
}