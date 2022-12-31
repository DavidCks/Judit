use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderBottomButtonStyle {
    position: String,
}

impl Style for BorderBottomButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderBottomButton {
   style: BorderBottomButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderBottomButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderBottomButton {
    type Message = Msg;
    type Properties = BorderBottomButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderBottomButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m20 4.01.01-.011M16 4.01l.01-.011M12 4.01l.01-.011M8 4.01l.01-.011M4 4.01l.01-.011M4 8.01l.01-.011M4 12.01l.01-.011m7.99.011.01-.011M4 16.01l.01-.011M20 8.01l.01-.011M20 12.01l.01-.011M20 16.01l.01-.011M4 20h16"/>
            </svg>
        }
    }
}