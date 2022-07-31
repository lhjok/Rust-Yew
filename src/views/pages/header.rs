use yew::prelude::*;
use crate::AppRoute;
use yew_router::prelude::*;

// Header组件
pub struct Header;

impl Component for Header {
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
            <div id="header">
                <div class="logo">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <img src="/static/image/title.png"/>
                    </Link<AppRoute>>
                </div>
            </div>
        }
    }
}
