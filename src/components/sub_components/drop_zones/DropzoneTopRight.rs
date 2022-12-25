use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneTopRightStyle {
    position: String,
    right: String,
    top: String,
}

impl Style for DropzoneTopRightStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                right: "0px",
                top: "0px",
            }
        )
    }
}

pub struct DropzoneTopRight {
    hover_class: String,
    style: DropzoneTopRightStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneTopRightProps {
    #[prop_or_default]
    pub onmouseup: Callback<MouseEvent>,
}

impl Component for DropzoneTopRight {
    type Message = Msg;
    type Properties = DropzoneTopRightProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneTopRightStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onmouseup={ ctx.props().onmouseup.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m7.99 4 .011.01M3.99 4l.011.01M3.99 8l.011.01M3.99 12l.011.01M3.99 16l.011.01M3.99 20l.011.01M7.99 20l.011.01M11.99 20l.011.01M15.99 20l.011.01M19.99 20l.011.01M19.99 16l.011.01M12 4h8v8h-8V4Zm3 4h1m1 0h-1m0 0V7m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}