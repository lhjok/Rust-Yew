pub mod index;
pub mod config;
use yew::prelude::*;
pub use self::index::Index;
pub use self::config::Config;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

// Content容器组件
#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    html! {
        <div id="content">
            { for props.children.iter() }
        </div>
    }
}