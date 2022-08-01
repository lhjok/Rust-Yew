use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::AdminRoute;
use super::pages::{
    Aside, Footer, Header, Content,
    content::{ Index }
};

pub fn switch_admin(routes: &AdminRoute) -> Html {
    match routes {
        AdminRoute::Admin => html!{ 
            <Redirect<AdminRoute> to={AdminRoute::AdminIndex}/>
        },
        AdminRoute::AdminIndex => html!{ <Index/> },
        AdminRoute::NotFound => html!{ <h1>{ "404" }</h1> }
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
