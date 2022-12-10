use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct AlignLeftButtonStyle {
    position: String,
}

impl Style for AlignLeftButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct AlignLeftButton {
   style: AlignLeftButtonStyle
}

pub enum Msg {
}

impl Component for AlignLeftButton {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: AlignLeftButtonStyle::create(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <svg style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M3 10h14M3 6h18M3 18h14M3 14h18" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}