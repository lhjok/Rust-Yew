use yew::prelude::*;
use crate::{AppRoute, AdminRoute};
use yew_router::prelude::*;

// Aside组件
pub struct Aside;

impl Component for Aside {
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
            <div id="aside">
                <ul class="list">
                    <li>
                        <Link<AppRoute> to={AppRoute::Home}>
                            {"首页"}
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AdminRoute> to={AdminRoute::Admin}>
                            {"管理"}
                        </Link<AdminRoute>>
                    </li>
                </ul>
            </div>
        }
    }
}
