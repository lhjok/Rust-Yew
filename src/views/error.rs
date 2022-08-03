use yew::prelude::*;

// Error组件
pub struct Error;

impl Component for Error {
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
            <div id="error">
                <span class="hello-404"><h1>{"404"}</h1></span>
            </div>
        }
    }
}
