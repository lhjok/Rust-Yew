use yew_router::prelude::*;

// 一级路由设置
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

// 二级路由设置
#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin")]
    Admin,
    #[at("/admin/index")]
    AdminIndex,
    #[not_found]
    #[at("/admin/404")]
    NotFound
}