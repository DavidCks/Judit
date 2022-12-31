use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderBottomRightButtonStyle {
    position: String,
}

impl Style for BorderBottomRightButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderBottomRightButton {
   style: BorderBottomRightButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderBottomRightButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderBottomRightButton {
    type Message = Msg;
    type Properties = BorderBottomRightButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderBottomRightButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M20.01 4v16h-16M4 16l.012.01M4 12l.012.01M4 8l.012.01M4 4l.012.01M16 4l.012.01M12 4l.012.01M8 4l.012.01"/>
            </svg>
        }
    }
}