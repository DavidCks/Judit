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
                top: "calc(100% + 2px)",
                left: "0px",
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
            <svg class={ self.style.as_class(&document).unwrap() } jrole="Judit_Transform3DToggle" width="24px" height="24px" stroke_width="1.5" viewBox="0 0 24 24" fill="white" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path jrole="Judit_Transform3DToggle" d="M21 7.353v9.294a.6.6 0 01-.309.525l-8.4 4.666a.6.6 0 01-.582 0l-8.4-4.666A.6.6 0 013 16.647V7.353a.6.6 0 01.309-.524l8.4-4.667a.6.6 0 01.582 0l8.4 4.667a.6.6 0 01.309.524z" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round"></path>
                <path jrole="Judit_Transform3DToggle" d="M3.528 7.294l8.18 4.544a.6.6 0 00.583 0l8.209-4.56M12 21v-9" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round"></path>
            </svg>
        }
    }
}