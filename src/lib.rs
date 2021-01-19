#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]
use wasm_bindgen::prelude::*;
use app::App;
pub mod app;
pub mod views;

// 使用 wee_alloc 全局分配器
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// WEB应用程序入口
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
