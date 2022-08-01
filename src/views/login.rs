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
        true
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div id="login">
                <span class="hello-world"><h1>{"Login"}</h1></span>
            </div>
        }
    }
}
