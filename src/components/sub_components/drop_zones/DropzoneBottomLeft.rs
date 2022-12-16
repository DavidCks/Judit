use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneBottomLeftStyle {
    position: String,
    left: String,
    bottom: String,
}

impl Style for DropzoneBottomLeftStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                left: "0px",
                bottom: "0px",
            }
        )
    }
}

pub struct DropzoneBottomLeft {
    hover_class: String,
    style: DropzoneBottomLeftStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneBottomLeftProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for DropzoneBottomLeft {
    type Message = Msg;
    type Properties = DropzoneBottomLeftProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneBottomLeftStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m16.01 20-.011-.01m4.011.01-.011-.01M20.01 16l-.011-.01M20.01 12l-.011-.01M20.01 8l-.011-.01M20.01 4l-.011-.01M16.01 4l-.011-.01M12.01 4l-.011-.01M8.01 4l-.011-.01M4.01 4l-.011-.01M4.01 8l-.011-.01M12 20H4v-8h8v8Zm-5-4h1m1 0H8m0 0v-1m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}