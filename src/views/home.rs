use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::{AppRoute, AdminRoute};

pub fn switch_main(routes: &AppRoute) -> Html {
    match routes {
        _ => html!{
            <Redirect<AdminRoute> to={AdminRoute::AdminIndex} />
        }
    }
}

// Home组件
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div id="home">
            <Switch<AppRoute> render={Switch::render(switch_main)} />
        </div>
    }
}