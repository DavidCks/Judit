use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct DirectionRightLeftHorizontalWordsStyle {
    position: String,
}

impl Style for DirectionRightLeftHorizontalWordsStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct DirectionRightLeftHorizontalWords {
   style: DirectionRightLeftHorizontalWordsStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DirectionRightLeftHorizontalWordsProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for DirectionRightLeftHorizontalWords {
    type Message = Msg;
    type Properties = DirectionRightLeftHorizontalWordsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: DirectionRightLeftHorizontalWordsStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M21.122 12h-12m12 5h-6m-12-10h18m-18 0 2.117 2M3.122 7l2.117-2" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}