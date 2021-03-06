use yew::prelude::*;
use crate::views::admin::AdminRoute;
use yew_router::prelude::*;
use css_in_rust::Style;

// Header组件
pub struct Header {
    style: Style
}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let css = include_str!("../../../static/styles/header.css");
        let style = match Style::create("header", css) {
            Ok(style) => style,
            Err(error) => { panic!("Error Style: {}", error); }
        };
        Self {
            style
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                <div class="logo">
                    <RouterAnchor<AdminRoute> route=AdminRoute::Admin>
                        <img src="image/title.png"/>
                    </RouterAnchor<AdminRoute>>
                </div>
            </div>
        }
    }
}