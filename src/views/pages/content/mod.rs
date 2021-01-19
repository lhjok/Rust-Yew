pub mod index;
use yew::prelude::*;
pub use self::index::Index;

// Content容器组件
pub struct Content {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

impl Component for Content {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Content {
            props
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
            <div id="content">
                { self.props.children.clone() }
            </div>
        }
    }
}