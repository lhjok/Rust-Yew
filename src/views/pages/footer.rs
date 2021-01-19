use yew::prelude::*;
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
                <p>{"千鸟科技 - 瑞金市千鸟网络科技有限公司 CopyRight 2017-2020"}
                <span>{" - 赣ICP备17011754号-2 - "}</span>{" 软件著作权 - "}
                <span>{"软著登字第8888888号"}</span></p>
            </div>
        }
    }
}