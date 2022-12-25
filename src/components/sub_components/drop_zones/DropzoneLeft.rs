use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneLeftStyle {
    position: String,
    width: String,
    height: String,
    margin: String,
    left: String,
    top: String,
    bottom: String,
}

impl Style for DropzoneLeftStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                width: "24px",
                height: "24px",
                margin: "auto",
                top: "0",
                left: "0",
                bottom: "0",
            }
        )
    }
}

pub struct DropzoneLeft {
    hover_class: String,
    style: DropzoneLeftStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneLeftProps {
    #[prop_or_default]
    pub onmouseup: Callback<MouseEvent>,
}

impl Component for DropzoneLeft {
    type Message = Msg;
    type Properties = DropzoneLeftProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneLeftStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onmouseup={ ctx.props().onmouseup.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m16.004 3.995-.011.01m4.011-.01-.011.01m.011 3.99-.011.01m.011 3.99-.011.01m.011 3.99-.011.01m.011 3.99-.011.01m-3.989-.01-.011.01m-3.987-16.01h-8v16h8v-16ZM7 12h1m1 0H8m0 0v-1m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}