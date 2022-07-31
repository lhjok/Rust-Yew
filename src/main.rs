mod views;
use yew::prelude::*;
use yew_router::prelude::*;
use views::{ Admin, Login };

// 一级路由设置
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin")]
    Admin,
    // 二级路由设置
    #[at("/admin/:route")]
    AdminRoute,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_main(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html!{
            <Redirect<AppRoute> to={AppRoute::Admin}/>
        },
        AppRoute::Login => html!{ <Login/> },
        AppRoute::Admin => html!{ <Admin/> },
        AppRoute::AdminRoute => html!{ <Admin/> },
        AppRoute::NotFound => html! { <h1>{ "404" }</h1> }
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
