use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct StyleItalicButtonStyle {
    position: String,
}

impl Style for StyleItalicButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct StyleItalicButton {
   style: StyleItalicButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct StyleItalicButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for StyleItalicButton {
    type Message = Msg;
    type Properties = StyleItalicButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: StyleItalicButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M11 5h3m3 0h-3m0 0l-4 14m0 0H7m3 0h3" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}