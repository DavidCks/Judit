use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct RightAlignedVerticalTextStyle {
    position: String,
}

impl Style for RightAlignedVerticalTextStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct RightAlignedVerticalText {
   style: RightAlignedVerticalTextStyle
}

pub enum Msg {
}
#[derive(Properties, PartialEq)]
pub struct RightAlignedVerticalTextProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for RightAlignedVerticalText {
    type Message = Msg;
    type Properties = RightAlignedVerticalTextProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: RightAlignedVerticalTextStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M11.122 6.122v8m-5-8v4m10 7.878V6m0 12-2-2.118m2 2.118 2-2.118" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}