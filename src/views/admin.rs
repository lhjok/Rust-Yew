use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::{AppRoute, AdminRoute};
use super::pages::{
    Aside, Footer, Header, Content, content::Index
};

pub fn switch_admin(routes: &AdminRoute) -> Html {
    match routes {
        AdminRoute::Admin => html!{ 
            <Redirect<AdminRoute> to={AdminRoute::AdminIndex} />
        },
        AdminRoute::AdminIndex => html!{ <Index/> },
        AdminRoute::NotFound => html!{
            <Redirect<AppRoute> to={AppRoute::NotFound} />
        }
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
                <Switch<AdminRoute> render={Switch::render(switch_admin)} />
            </Content>
            <Footer/>
        </div>
    }
}
