use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderTopLeftButtonStyle {
    position: String,
}

impl Style for BorderTopLeftButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderTopLeftButton {
   style: BorderTopLeftButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderTopLeftButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderTopLeftButton {
    type Message = Msg;
    type Properties = BorderTopLeftButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderTopLeftButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m8 20.01.01-.011m3.99.011.01-.011m3.99.011.01-.011m3.99.011.01-.011M20 16.01l.01-.011M20 12.01l.01-.011M20 8.01l.01-.011M4 20V4h16"/>
            </svg>
        }
    }
}