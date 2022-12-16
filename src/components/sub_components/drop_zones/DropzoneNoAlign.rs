use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneNoAlignStyle {
    position: String,
    width: String,
    height: String,
    margin: String,
    left: String,
    right: String,
    bottom: String,
    top: String,
}

impl Style for DropzoneNoAlignStyle {
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
                top: "0",
            }
        )
    }
}

pub struct DropzoneNoAlign {
    style: DropzoneNoAlignStyle,
    hover_class: String,
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneNoAlignProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for DropzoneNoAlign {
    type Message = Msg;
    type Properties = DropzoneNoAlignProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneNoAlignStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m16.828 12.404-4.217 4.353c-.313.324-.909.324-1.222 0l-4.217-4.353a.561.561 0 0 1 0-.808l4.217-4.353c.313-.324.909-.324 1.222 0l4.217 4.353a.561.561 0 0 1 0 .808ZM12 22v-2m0-16V2M4 12H2m20 0h-2m-9 0h1m1 0h-1m0 0v-1m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}