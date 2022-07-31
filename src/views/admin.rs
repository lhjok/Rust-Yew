use yew::prelude::*;
use yew_router::prelude::*;
use super::pages::{
    Aside, Footer, Header, Content,
    content::{ Index }
};

// 二级路由设置
#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin")]
    Admin,
    #[at("/admin/index")]
    AdminIndex,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_admin(routes: &AdminRoute) -> Html {
    match routes {
        AdminRoute::Admin => html!{
            <Redirect<AdminRoute> to={AdminRoute::AdminIndex}/>
        },
        AdminRoute::AdminIndex => html!{ <Index/> },
        AdminRoute::NotFound => html! { <h1>{ "404" }</h1> }
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
