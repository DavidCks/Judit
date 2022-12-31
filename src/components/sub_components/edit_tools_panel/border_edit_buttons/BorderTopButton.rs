use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderTopButtonStyle {
    position: String,
}

impl Style for BorderTopButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderTopButton {
   style: BorderTopButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderTopButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderTopButton {
    type Message = Msg;
    type Properties = BorderTopButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderTopButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m20.986 20.08.01-.012m-4.024.012.01-.012m-4.024.012.01-.012m-4.024.012.01-.012m-4.024.012.01-.012M4.93 8.037l.01-.01m-.01 4.024.01-.01m8.018.01.01-.01M4.93 16.065l.01-.01m16.046-8.018.01-.01m-.01 4.024.01-.01m-.01 4.024.01-.01M4.93 4.014h16.056"/>
            </svg>
        }
    }
}