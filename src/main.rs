mod views;
mod route;
use yew::prelude::*;
use yew_router::prelude::*;
use route::AppRoute;
use views::{ Admin, Login };

pub fn switch_main(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html!{ 
            <Redirect<AppRoute> to={AppRoute::Admin}/>
        },
        AppRoute::Login => html!{ <Login/> },
        AppRoute::Admin => html!{ <Admin/> },
        AppRoute::AdminRoute => html!{ <Admin/> },
        AppRoute::NotFound => html!{ <h1>{ "404" }</h1> }
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
