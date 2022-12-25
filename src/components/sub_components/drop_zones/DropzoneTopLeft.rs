use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneTopLeftStyle {
    position: String,
    left: String,
    top: String,
}

impl Style for DropzoneTopLeftStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                left: "0px",
                top: "0px",
            }
        )
    }
}

pub struct DropzoneTopLeft {
    hover_class: String,
    style: DropzoneTopLeftStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneTopLeftProps {
    #[prop_or_default]
    pub onmouseup: Callback<MouseEvent>,
}

impl Component for DropzoneTopLeft {
    type Message = Msg;
    type Properties = DropzoneTopLeftProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneTopLeftStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onmouseup={ ctx.props().onmouseup.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m4 16.01.01-.011M4 20.01l.01-.011M8 20.01l.01-.011m3.99.011.01-.011m3.99.011.01-.011m3.99.011.01-.011M20 16.01l.01-.011M20 12.01l.01-.011M20 8.01l.01-.011M20 4.01l.01-.011M16 4.01l.01-.011M4 12V4h8v8H4Zm3-4h1m1 0H8m0 0V7m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}