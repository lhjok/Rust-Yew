use yew::prelude::*;
use yew_router::{ prelude::*, Switch };
use crate::views::{ Admin, Login };

// 项目根组件
pub struct App;
// 路由设置
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/#/login"]
    Login,
    #[to = "/#/admin"]
    Admin
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App{}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Login => html!{ <Login/> },
                        AppRoute::Admin => html!{ <Admin/> }
                    }
                })
                redirect = Router::redirect(|_|AppRoute::Admin)
            />
        }
    }
}
