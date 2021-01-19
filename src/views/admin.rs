use yew::prelude::*;
use yew_router::{ prelude::*, Switch };
use crate::views::pages::{
    Aside, Footer, Header, Content,
    content::{ Index }
};

// Admin组件
pub struct Admin;

#[derive(Switch, Debug, Clone)]
pub enum AdminRoute {
    #[to = "/#/admin/index"]
    AdminIndex,
    #[to = "/#/admin"]
    Admin
}

impl Component for Admin {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Admin{}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="app">
                <Header/>
                <Aside/>
                <Content>
                    <Router<AdminRoute>
                        render = Router::render(|switch: AdminRoute| {
                            match switch {
                                AdminRoute::AdminIndex => html!{ <Index/> },
                                AdminRoute::Admin => html!{ <Index/> }
                            }
                        })
                    />
                </Content>
                <Footer/>
            </div>
        }
    }
}