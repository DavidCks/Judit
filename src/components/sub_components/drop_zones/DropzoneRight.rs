use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneRightStyle {
    position: String,
    width: String,
    height: String,
    margin: String,
    right: String,
    top: String,
    bottom: String,
}

impl Style for DropzoneRightStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                width: "24px",
                height: "24px",
                margin: "auto",
                top: "0",
                right: "0",
                bottom: "0",
            }
        )
    }
}

pub struct DropzoneRight {
    hover_class: String,
    style: DropzoneRightStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneRightProps {
    #[prop_or_default]
    pub onmouseup: Callback<MouseEvent>,
}

impl Component for DropzoneRight {
    type Message = Msg;
    type Properties = DropzoneRightProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneRightStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onmouseup={ ctx.props().onmouseup.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m8.006 20.005.01-.01m-4.01.01.01-.01m-.01-3.99.01-.01m-.01-3.99.01-.01m-.01-3.99.01-.01m-.01-3.99.01-.01m3.99.01.01-.01m3.99 16.01h8v-16h-8v16ZM15 12h1m1 0h-1m0 0v-1m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}