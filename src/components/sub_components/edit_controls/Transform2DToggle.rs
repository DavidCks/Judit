use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

#[allow(non_snake_case)]
#[derive(Reflect)]
struct Transform2DToggleStyle {
    position: String,
    width: String,
    height: String,
    display: String,
    align_items: String,
    justify_content: String,
}

impl Style for Transform2DToggleStyle {
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

pub struct Transform2DToggle {
   style: Transform2DToggleStyle,
   hover_style: Transform3DToggleStyle_hover,
}

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Reflect)]
struct Transform3DToggleStyle_hover {
    append: String,
    cursor: String,
}

impl Style for Transform3DToggleStyle_hover {
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
pub struct Transform2DToggleProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Transform2DToggle {
    type Message = Msg;
    type Properties = Transform2DToggleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: Transform2DToggleStyle::create(),
            hover_style: Transform3DToggleStyle_hover::create(),
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
                jrole="Judit_Transform2DToggle" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path jrole="Judit_Transform2DToggle" d="M21 3.6v16.8a.6.6 0 01-.6.6H3.6a.6.6 0 01-.6-.6V3.6a.6.6 0 01.6-.6h16.8a.6.6 0 01.6.6z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
        }
    }
}