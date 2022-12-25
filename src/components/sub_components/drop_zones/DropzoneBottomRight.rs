use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

use super::DropzoneHoverStyle::DropzoneHoverStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DropzoneBottomRightStyle {
    position: String,
    right: String,
    bottom: String
}

impl Style for DropzoneBottomRightStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                right: "0px",
                bottom: "0px",
            }
        )
    }
}

pub struct DropzoneBottomRight {
    hover_class: String,
    style: DropzoneBottomRightStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DropzoneBottomRightProps {
    #[prop_or_default]
    pub onmouseup: Callback<MouseEvent>,
}

impl Component for DropzoneBottomRight {
    type Message = Msg;
    type Properties = DropzoneBottomRightProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let document = window().unwrap().document().unwrap();
        Self {
            hover_class: DropzoneHoverStyle::create().as_class(&document).unwrap(),
            style: DropzoneBottomRightStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onmouseup={ ctx.props().onmouseup.clone() } style={ self.style.inline() } class={ &self.hover_class } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="m20 7.99-.01.011M20 3.99l-.01.011M16 3.99l-.01.011M12 3.99l-.01.011M8 3.99l-.01.011M4 3.99l-.01.011M4 7.99l-.01.011M4 11.99l-.01.011M4 15.99l-.01.011M4 19.99l-.01.011M8 19.99l-.01.011M20 12v8h-8v-8h8Zm-5 4h1m1 0h-1m0 0v-1m0 1v1" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}