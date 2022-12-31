use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderRightButtonStyle {
    position: String,
}

impl Style for BorderRightButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderRightButton {
   style: BorderRightButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderRightButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderRightButton {
    type Message = Msg;
    type Properties = BorderRightButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderRightButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m4 20 .012.01M4 16l.012.01M4 12l.012.01M4 8l.012.01M4 4l.012.01M16 4l.012.01M12 4l.012.01M12 12l.012.01M8 4l.012.01M16 20l.012.01M12 20l.012.01M8 20l.012.01M20.01 4v16"/>
            </svg>
        }
    }
}