use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderResizeButtonStyle {
    position: String,
}

impl Style for BorderResizeButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderResizeButton {
   style: BorderResizeButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderResizeButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderResizeButton {
    type Message = Msg;
    type Properties = BorderResizeButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderResizeButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg jrole="Judit_BorderResizeButton" onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path jrole="Judit_BorderResizeButton" stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 9.5 9.5 12 7 14.5m9.5-5L14 12l2.5 2.5"/>
                <path jrole="Judit_BorderResizeButton" stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M6 5h12a4 4 0 0 1 4 4v6a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V9a4 4 0 0 1 4-4Z"/>
            </svg>
        }
    }
}