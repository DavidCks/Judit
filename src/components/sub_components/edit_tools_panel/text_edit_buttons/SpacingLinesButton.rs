use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct SpacingLinesButtonStyle {
    position: String,
}

impl Style for SpacingLinesButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct SpacingLinesButton {
   style: SpacingLinesButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct SpacingLinesButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for SpacingLinesButton {
    type Message = Msg;
    type Properties = SpacingLinesButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: SpacingLinesButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M11 7h9M11 12h9M11 17h9M6 17V7m0 10l-2-2.5M6 17l2-2.5M6 7L4 9m2-2l2 2" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}