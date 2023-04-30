use yew::prelude::*;

// Error组件
#[function_component(Error)]
pub fn error() -> Html {
    html! {
        <div id="error">
            <h1>{"404"}</h1>
        </div>
    }
}