use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct StyleBoldButtonStyle {
    position: String,
}

impl Style for StyleBoldButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct StyleBoldButton {
   style: StyleBoldButtonStyle
}

pub enum Msg {
}

impl Component for StyleBoldButton {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: StyleBoldButtonStyle::create(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <svg style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path d="M12 11.667H8m4 0s3.333 0 3.333-3.334C15.333 5 12 5 12 5s0 0 0 0H8.6a.6.6 0 00-.6.6v6.067m4 0s4 0 4 3.666C16 19 12 19 12 19s0 0 0 0H8.6a.6.6 0 01-.6-.6v-6.733" stroke="#000000" stroke-width="1.5"></path>
            </svg>
        }
    }
}