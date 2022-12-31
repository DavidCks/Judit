use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderLeftButtonStyle {
    position: String,
}

impl Style for BorderLeftButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderLeftButton {
   style: BorderLeftButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderLeftButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderLeftButton {
    type Message = Msg;
    type Properties = BorderLeftButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderLeftButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m20.011 20-.012.01m.012-4.01-.012.01m.012-4.01-.012.01M20.011 8 20 8.01M20.011 4 20 4.01M8.011 4 8 4.01M12.011 4 12 4.01m.011 7.99-.012.01M16.011 4 16 4.01M8.011 20 8 20.01m4.011-.01-.012.01m4.012-.01-.012.01M4 4v16"/>
            </svg>
        }
    }
}