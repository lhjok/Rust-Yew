use yew::prelude::*;
use yew_router::prelude::*;
use crate::views::Error;
use crate::route::AdminRoute;
use super::pages::{
    Aside, Footer, Header, Content, content::Index
};

pub fn switch_main(routes: &AdminRoute) -> Html {
    match routes {
        AdminRoute::AdminIndex => html!{ <Index/> },
        AdminRoute::NotFound => html!{ <Error/> }
    }
}

// Admin组件
#[function_component(Admin)]
pub fn admin() -> Html {
    html! {
        <div id="admin">
            <Header/>
            <Aside/>
            <Content>
                <Switch<AdminRoute> render={Switch::render(switch_main)} />
            </Content>
            <Footer/>
        </div>
    }
}
