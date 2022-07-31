use yew::prelude::*;

// Footer组件
pub struct Footer;

impl Component for Footer {
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
            <div id="footer">
                <p>{"千鸟科技 - 瑞金市千鸟网络科技有限公司 CopyRight 2017-2020"}
                <span>{" - 赣ICP备17011754号-2 - "}</span>{" 软件著作权 - "}
                <span>{"软著登字第8888888号"}</span></p>
            </div>
        }
    }
}
