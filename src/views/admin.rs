use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::{AppRoute, AdminRoute};
use super::pages::{
    Aside, Footer, Header, Content, content::Index
};

pub fn switch_main(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Admin => html!{ 
            <Redirect<AdminRoute> to={AdminRoute::AdminIndex} />
        },
        AppRoute::AdminRoute | _ => html!{ 
            <Switch<AdminRoute> render={Switch::render(switch_admin)} />
        }
    }
}

pub fn switch_admin(routes: &AdminRoute) -> Html {
    match routes {
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
                <Switch<AppRoute> render={Switch::render(switch_main)} />
            </Content>
            <Footer/>
        </div>
    }
}
