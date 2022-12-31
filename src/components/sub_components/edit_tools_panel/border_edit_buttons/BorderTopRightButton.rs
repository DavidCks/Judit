use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderTopRightButtonStyle {
    position: String,
}

impl Style for BorderTopRightButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderTopRightButton {
   style: BorderTopRightButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderTopRightButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderTopRightButton {
    type Message = Msg;
    type Properties = BorderTopRightButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderTopRightButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M20.01 20V4h-16M4 8l.012-.01M4 12l.012-.01M4 16l.012-.01M4 20l.012-.01M16 20l.012-.01M12 20l.012-.01M8 20l.012-.01"/>
            </svg>
        }
    }
}