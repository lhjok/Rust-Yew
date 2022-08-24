use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::{AppRoute, AdminRoute};

// Home组件
pub struct Home;

pub fn switch_main(routes: &AppRoute) -> Html {
    match routes {
        _ => html!{
            <Redirect<AdminRoute> to={AdminRoute::AdminIndex} />
        }
    }
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div id="home">
                <Switch<AppRoute> render={Switch::render(switch_main)} />
            </div>
        }
    }
}
