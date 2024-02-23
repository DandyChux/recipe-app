mod api;
mod store;
mod components;

use yew::prelude::*;
use yewdux::prelude::*;
use store::Store;
use components::{
    alert::{AlertComponent, AlertProps}
};

#[function_component]
fn App() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = &store.loading;

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <>
            if show_alert {
                <AlertComponent
                    message={alert_props.message}
                    delay_ms={alert_props.delay_ms}
                />
            }
        
            <main class="px-5 py-12 md:container">
                <div class="flex flex-col items-center gap-4">
                    <p>{ *counter }</p>
                    <button {onclick} class="px-8 py-4 transition-all duration-500 border border-black rounded-md hover:bg-primary">{ "+1" }</button>
                </div>
            </main>

            if *loading {
                <div
                    class="fixed top-5 left-5 inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-warning border-r-transparent align-[-0.125em] text-warning motion-reduce:animate-[spin_1.5s_linear_infinite]"
                    role="status"
                >
                    <span
                        class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                    >
                        {"Loading..."}
                    </span>
                </div>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}