use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ window };

#[allow(non_snake_case)]
#[derive(Reflect)]
struct Transform3DToggleStyle {
    position: String,
    width: String,
    height: String,
    top: String,
    left: String,
    display: String,
    align_items: String,
    justify_content: String,
}

impl Style for Transform3DToggleStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                width: "30px",
                height: "30px",
                top: "calc(50% - 15px)",
                left: "calc(50% - 15px)",
                display: "flex",
                align_items: "center",
                justify_content: "center",
            }
        )
    }
}

#[allow(non_snake_case)]
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

pub struct Transform3DToggle {
   style: Transform3DToggleStyle,
   hover_style: Transform3DToggleStyle_hover,
}

pub enum Msg {
}

impl Component for Transform3DToggle {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            style: Transform3DToggleStyle::create(),
            hover_style: Transform3DToggleStyle_hover::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let window = window().expect("No global `window` found");
        let document = window.document().expect("couldn't get `document");

        html! {
            <span class={ self.hover_style.as_class(&document).unwrap() } jrole="Judit_Transform3DToggle" style={ self.style.inline() }>
                { "3D" }
            </span> 
        }
    }
}