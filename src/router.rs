use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/meetup/:id")]
    Meetup { id: String },
}

#[derive(Clone, PartialEq, Properties)]
pub struct AppLinkProps {
    pub route: AppRoute,
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(AppLink)]
pub fn app_link(props: &AppLinkProps) -> Html {
    let AppLinkProps {
        route,
        children,
        classes,
    } = props;
    let navigator = use_navigator().expect("Navigator not found");
    html! {
        <button
            class={classes.clone()}
            onclick={
            let route = route.clone();
            Callback::from(move |_| {
            navigator.push(&route);
            })}>
            {children}
        </button>
    }
}
