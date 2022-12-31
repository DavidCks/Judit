use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderVisibilityToggleStyle {
    position: String,
}

impl Style for BorderVisibilityToggleStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
            }
        )
    }
}

pub struct BorderVisibilityToggle {
   style: BorderVisibilityToggleStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderVisibilityToggleProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(true)]
    pub is_border_visible: bool,
}

impl Component for BorderVisibilityToggle {
    type Message = Msg;
    type Properties = BorderVisibilityToggleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderVisibilityToggleStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg onclick={ ctx.props().onclick.clone() } style={ self.style.inline() } width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
            if ctx.props().is_border_visible {
                <path stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m3 3 18 18M10.5 10.677a2 2 0 0 0 2.823 2.823"/>
                <path stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7.362 7.561C5.68 8.74 4.279 10.42 3 12c1.889 2.991 5.282 6 9 6 1.55 0 3.043-.523 4.395-1.35M12 6c4.008 0 6.701 3.158 9 6a15.66 15.66 0 0 1-1.078 1.5"/>
            } else {
                <path stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"/>
                <path stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 12c-1.889 2.991-5.282 6-9 6s-7.111-3.009-9-6c2.299-2.842 4.992-6 9-6s6.701 3.158 9 6Z"/>
            }
            </svg>
        }
    }
}