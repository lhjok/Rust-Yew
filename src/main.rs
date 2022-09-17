mod views;
mod route;
use yew::prelude::*;
use yew_router::prelude::*;
use route::{AppRoute, AdminRoute};
use views::{ Admin, Login, Home, Error };

pub fn switch_main(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html!{ <Home/> },
        AppRoute::Login => html!{ <Login/> },
        AppRoute::Admin => html!{
            <Redirect<AdminRoute> to={AdminRoute::AdminIndex} />
        },
        AppRoute::AdminRoute => html!{ <Admin/> },
        AppRoute::NotFound => html!{ <Error/> }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch_main)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
