use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneTopStyle {
    position: String,
    width: String,
    height: String,
    margin: String,
    left: String,
    right: String,
    top: String,
}

impl Style for DropzoneTopStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                width: "24px",
                height: "24px",
                margin: "auto",
                left: "0",
                right: "0",
                top: "0",
            }
        )
    }
}

pub struct DropzoneTop {
    hover_class: String,
    style: DropzoneTopStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneTopProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for DropzoneTop {
    type Message = Msg;
    type Properties = DropzoneTopProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneTopStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m4 16 .01-.011M4 20l.01-.011M8 20l.01-.011M12 20l.01-.011M16 20l.01-.011M20 20l.01-.011M20 16l.01-.011M4 12V4h16v8H4Zm7-4h1m1 0h-1m0 0V7m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}