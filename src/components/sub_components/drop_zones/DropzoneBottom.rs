use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneBottomStyle {
    position: String,
    width: String,
    height: String,
    margin: String,
    left: String,
    right: String,
    bottom: String,
}

impl Style for DropzoneBottomStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                width: "24px",
                height: "24px",
                margin: "auto",
                bottom: "0",
                left: "0",
                right: "0",
            }
        )
    }
}

pub struct DropzoneBottom {
    style: DropzoneBottomStyle,
    hover_class: String,
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneBottomProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for DropzoneBottom {
    type Message = Msg;
    type Properties = DropzoneBottomProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneBottomStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m4 8 .01.011M4 4l.01.011M8 4l.01.011M12 4l.01.011M16 4l.01.011M20 4l.01.011M20 8l.01.011M4 12v8h16v-8H4Zm7 4h1m1 0h-1m0 0v-1m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}