use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct SpacingLettersButtonStyle {
    position: String,
}

impl Style for SpacingLettersButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct SpacingLettersButton {
   style: SpacingLettersButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct SpacingLettersButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for SpacingLettersButton {
    type Message = Msg;
    type Properties = SpacingLettersButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: SpacingLettersButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M4.235 7v10m15.53-10v10m-2.824-5H7.06m9.882 0-2.117 2.118M16.94 12l-2.117-2.118M7.058 12l2.117 2.118M7.06 12l2.117-2.118" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}