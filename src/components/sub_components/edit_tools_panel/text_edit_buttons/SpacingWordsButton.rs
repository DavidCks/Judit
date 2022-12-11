use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct SpacingWordsButtonStyle {
    position: String,
}

impl Style for SpacingWordsButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct SpacingWordsButton {
   style: SpacingWordsButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct SpacingWordsButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for SpacingWordsButton {
    type Message = Msg;
    type Properties = SpacingWordsButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: SpacingWordsButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } jrole="Judit_SpacingWordsButton" style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M4 7h2.824m3.764.04h2.824M17 7h2.823m-.058 7.823H4.235m15.53 0-2.118 2.118m2.118-2.117-2.118-2.118M4.235 14.823l2.118 2.118m-2.118-2.117 2.118-2.118" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}