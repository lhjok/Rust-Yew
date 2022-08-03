use yew::prelude::*;

// Home组件
pub struct Home;

impl Component for Home {
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
            <div id="home">
                <span class="hello-home"><h1>{"Home"}</h1></span>
            </div>
        }
    }
}
