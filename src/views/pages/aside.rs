use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::{AppRoute, AdminRoute};

// Aside组件
#[function_component(Aside)]
pub fn aside() -> Html {
    html! {
        <div id="aside">
            <ul class="list">
                <li>
                    <Link<AppRoute> to={AppRoute::Home}>
                        {"首页"}
                    </Link<AppRoute>>
                </li>
                <li>
                    <Link<AdminRoute> to={AdminRoute::AdminIndex}>
                        {"管理"}
                    </Link<AdminRoute>>
                </li>
                <li>
                    <Link<AdminRoute> to={AdminRoute::AdminConfig}>
                        {"设置"}
                    </Link<AdminRoute>>
                </li>
            </ul>
        </div>
    }
}