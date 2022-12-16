use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct LeftAlignedVerticalTextStyle {
    position: String,
}

impl Style for LeftAlignedVerticalTextStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct LeftAlignedVerticalText {
   style: LeftAlignedVerticalTextStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct LeftAlignedVerticalTextProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for LeftAlignedVerticalText {
    type Message = Msg;
    type Properties = LeftAlignedVerticalTextProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: LeftAlignedVerticalTextStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M13 6.122v8m5-8v4M8 18V6m0 12 2-2.118M8 18l-2-2.118" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}