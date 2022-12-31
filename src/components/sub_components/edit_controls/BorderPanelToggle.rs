use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderPanelToggleStyle {
    position: String,
    width: String,
    height: String,
    display: String,
    align_items: String,
    justify_content: String,
}

impl Style for BorderPanelToggleStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "relative",
                width: "30px",
                height: "100%",
                display: "flex",
                align_items: "center",
                justify_content: "center",
            }
        )
    }
}

pub struct BorderPanelToggle {
   style: BorderPanelToggleStyle,
   hover_style: BorderPanelToggleStyle_hover,
}

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Reflect)]
struct BorderPanelToggleStyle_hover {
    append: String,
    cursor: String,
}

impl Style for BorderPanelToggleStyle_hover {
    fn create() -> Self {
        append_to_string!(
            Self {
                append: ":hover",
                cursor: "pointer",
            }
        )
    }
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderPanelToggleProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for BorderPanelToggle {
    type Message = Msg;
    type Properties = BorderPanelToggleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderPanelToggleStyle::create(),
            hover_style: BorderPanelToggleStyle_hover::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let window = window().expect("No global `window` found");
        let document = window.document().expect("couldn't get `document");

        html! {
            <svg class={ format!("{} {}", 
                self.style.as_class(&document).unwrap(),
                self.hover_style.as_class(&document).unwrap() ) } 
                onclick={ ctx.props().onclick.clone() }
                jrole="Judit_BorderPanelToggle" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path jrole="Judit_BorderPanelToggle" stroke="#3F3F3F" stroke-dasharray="2 2" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="1.5" stroke-width="1.499" d="M16 2H8a6 6 0 0 0-6 6v8a6 6 0 0 0 6 6h8a6 6 0 0 0 6-6V8a6 6 0 0 0-6-6Z"/>
                <path jrole="Judit_BorderPanelToggle" stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="1.5" stroke-width="1.499" d="M16 5H8a3 3 0 0 0-3 3v8a3 3 0 0 0 3 3h8a3 3 0 0 0 3-3V8a3 3 0 0 0-3-3Z"/>
            </svg>
        }
    }
}