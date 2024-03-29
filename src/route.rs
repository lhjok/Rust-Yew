use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin")]
    Admin,
    #[at("/admin/:route")]
    AdminRoute,
    #[not_found]
    #[at("/404")]
    NotFound
}

#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin/index")]
    AdminIndex,
    #[at("/admin/config")]
    AdminConfig,
    #[not_found]
    #[at("/admin/404")]
    NotFound
}