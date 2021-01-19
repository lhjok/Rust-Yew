use yew::prelude::*;
use crate::views::admin::AdminRoute;
use yew_router::prelude::*;
use css_in_rust::Style;

// Footer组件
pub struct Footer {
    style: Style
}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let css = include_str!("../../../static/styles/footer.css");
        let style = match Style::create("footer", css) {
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
                <RouterAnchor<AdminRoute> route=AdminRoute::AdminIndex>
                    <button>{"FooterIndex"}</button>
                </RouterAnchor<AdminRoute>>
            </div>
        }
    }
}