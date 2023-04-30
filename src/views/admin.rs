use yew::prelude::*;
use yew_router::prelude::*;
use crate::views::Error;
use crate::route::AdminRoute;
use super::pages::content::{
    Index, Config
};
use super::pages::{
    Aside, Footer, Header, Content
};

pub fn switch_main(routes: &AdminRoute) -> Html {
    match routes {
        AdminRoute::AdminIndex => html!{ <Index/> },
        AdminRoute::AdminConfig => html!{ <Config/> },
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