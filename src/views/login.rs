use yew::prelude::*;

// Login组件
pub struct Login;

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true // 指示组件应该重新渲染
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div id="login">
                <button>{"+"}</button>
                <span class="hello-world">{ "Login" }</span>
                <button>{"-"}</button>
            </div>
        }
    }
}
