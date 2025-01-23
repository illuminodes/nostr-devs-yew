use nostr_devs_yew::{AppLayout, AppRoute, HomePage, MeetupPage, TranslationProvider};

use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <TranslationProvider>
            <AppLayout>
                <Switch<AppRoute> render = { move |switch: AppRoute| {
                    match switch {
                        AppRoute::Home => html!{<HomePage />},
                        AppRoute::Meetup { id } => html!{<MeetupPage {id} />},
                    }
                }}/>
            </AppLayout>
            </TranslationProvider>
        </BrowserRouter>
    }
}
