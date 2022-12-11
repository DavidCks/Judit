use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct StyleSizeButtonStyle {
    position: String,
}

impl Style for StyleSizeButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct StyleSizeButton {
   style: StyleSizeButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct StyleSizeButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for StyleSizeButton {
    type Message = Msg;
    type Properties = StyleSizeButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: StyleSizeButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } jrole="Judit_StyleSizeButton" style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M18 21V11m0 10l-2-2.5m2 2.5l2-2.5M18 11l-2 2m2-2l2 2M9 5v12m0 0H7m2 0h2M15 7V5H3v2" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}