use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderAllButtonStyle {
    position: String,
}

impl Style for BorderAllButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderAllButton {
   style: BorderAllButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderAllButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderAllButton {
    type Message = Msg;
    type Properties = BorderAllButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderAllButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m12.01 16-.01.01m.01-4.01-.01.01M12.01 8l-.01.01M8.01 12l-.01.01m8.01-.01-.01.01m5-8.41v16.8a.6.6 0 0 1-.6.6H3.6a.6.6 0 0 1-.6-.6V3.6a.6.6 0 0 1 .6-.6h16.8a.6.6 0 0 1 .6.6Z"/>
            </svg>
        }
    }
}