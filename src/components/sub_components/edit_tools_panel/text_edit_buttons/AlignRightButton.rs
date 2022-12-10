use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct AlignRightButtonStyle {
    position: String,
}

impl Style for AlignRightButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct AlignRightButton {
   style: AlignRightButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct AlignRightButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for AlignRightButton {
    type Message = Msg;
    type Properties = AlignRightButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: AlignRightButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } jrole="Judit_AlignRight" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M7 10h14M3 6h18M7 18h14M3 14h18" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
            
        }
    }
}