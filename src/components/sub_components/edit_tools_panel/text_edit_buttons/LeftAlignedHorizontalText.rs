use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct LeftAlignedHorizontalTextStyle {
    position: String,
}

impl Style for LeftAlignedHorizontalTextStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct LeftAlignedHorizontalText {
   style: LeftAlignedHorizontalTextStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct LeftAlignedHorizontalTextProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for LeftAlignedHorizontalText {
    type Message = Msg;
    type Properties = LeftAlignedHorizontalTextProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: LeftAlignedHorizontalTextStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M3 12h12M3 17h6M21 7H3m18 0-2.118 2M21 7l-2.118-2" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}