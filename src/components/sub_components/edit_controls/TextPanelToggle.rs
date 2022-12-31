use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

#[allow(non_snake_case)]
#[derive(Reflect)]
struct TextPanelToggleStyle {
    position: String,
    width: String,
    height: String,
    display: String,
    align_items: String,
    justify_content: String,
}

impl Style for TextPanelToggleStyle {
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

pub struct TextPanelToggle {
   style: TextPanelToggleStyle,
   hover_style: TextPanelToggleStyle_hover,
}

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Reflect)]
struct TextPanelToggleStyle_hover {
    append: String,
    cursor: String,
}

impl Style for TextPanelToggleStyle_hover {
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
pub struct TextPanelToggleProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for TextPanelToggle {
    type Message = Msg;
    type Properties = TextPanelToggleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: TextPanelToggleStyle::create(),
            hover_style: TextPanelToggleStyle_hover::create(),
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
                jrole="Judit_TextPanelToggle" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path jrole="Judit_TextPanelToggle" stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 3.6v16.8a.6.6 0 0 1-.6.6H3.6a.6.6 0 0 1-.6-.6V3.6a.6.6 0 0 1 .6-.6h16.8a.6.6 0 0 1 .6.6Z"/>
                <path jrole="Judit_TextPanelToggle" stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 9V7h10v2m-5-2v10m0 0h-2m2 0h2"/>
            </svg>
        }
    }
}