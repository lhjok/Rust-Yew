use yew::prelude::*;
// 加载 CSS 模块化库
use css_in_rust::Style;
// 加载 Yew 的 UI 库
use yew_styles::{
    button::Button,
    styles::{Palette, Size, Style as Styles}
};

// Login组件
pub struct Login {
    link: ComponentLink<Self>,
    style: Style,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let css = include_str!("../../static/styles/login.css");
        let style = match Style::create("login", css) {
            Ok(style) => style,
            Err(error) => { panic!("Error Style: {}", error); }
        };
        Self {
            link,
            style,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => self.value += 1,
            Msg::Decrement => self.value -= 1
        }
        true // 指示组件应该重新渲染
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                <Button
                    onclick_signal=self.link.callback(|_| Msg::Increment)
                    class_name="hello-world"
                    button_palette=Palette::Standard
                    button_style=Styles::Regular
                    button_size=Size::Big
                >{"+"}</Button>
                <span class="hello-world">{ self.value }</span>
                <Button
                    onclick_signal=self.link.callback(|_| Msg::Decrement)
                    class_name="hello-world"
                    button_palette=Palette::Standard
                    button_style=Styles::Regular
                    button_size=Size::Big
                >{"-"}</Button>
            </div>
        }
    }
}