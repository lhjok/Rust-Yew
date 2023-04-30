use yew::prelude::*;
use crate::AppRoute;
use yew_router::prelude::*;

// Header组件
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div id="header">
            <div class="logo">
                <Link<AppRoute> to={AppRoute::Home}>
                    <img src="/static/image/title.png"/>
                </Link<AppRoute>>
            </div>
        </div>
    }
}