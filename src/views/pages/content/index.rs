use yew::prelude::*;
// Index组件
pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true  // 指示组件应该重新渲染
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div id="contentindex">
                <button>{"+"}</button>
                <span class="hello-world">{ "Index" }</span>
                <button>{"-"}</button>
            </div>
        }
    }
}
