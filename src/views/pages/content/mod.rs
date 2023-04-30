pub mod index;
pub mod config;
use yew::prelude::*;
pub use self::index::Index;
pub use self::config::Config;

// Content容器组件
pub struct Content;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children,
}

impl Component for Content {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Content {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="content">
                { for ctx.props().children.iter() }
            </div>
        }
    }
}