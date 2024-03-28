use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;
use crate::components::{
    alert::{AlertComponent, AlertProps},
    ui::spinner::Spinner,
    header::Header
};

use crate::router::{Route, switch};
use crate::store::Store;

#[function_component(App)]
pub fn app() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let is_page_loading = &store.loading;

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <BrowserRouter>
            <div class="relative flex flex-col h-full lg:flex-row lg:flex-1">
                <Header class="w-full lg:basis-[20%] 2xl:basis-[25%]" />
                <main class="lg:basis-[80%] 2xl:basis-[75%]">
                    <Switch<Route> render={switch} />
                </main>
            </div>
            
            if show_alert {
                <AlertComponent
                    message={alert_props.message}
                    delay_ms={alert_props.delay_ms}
                />
            }

            if *is_page_loading {
                <div class="pt-4 pl-2 top-[5.5rem] fixed">
                    <Spinner width={"1.5rem"} height={"1.5rem"} color="text-secondary" />
                </div>
            }
        </BrowserRouter>
    }
}