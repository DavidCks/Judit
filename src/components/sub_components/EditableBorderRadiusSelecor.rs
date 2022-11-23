use rusty_css::*;
use yew::{prelude::*};
use web_sys::{ window };
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct EditableBorderRadiusSelectorStyle_hover {
    append: String,
    background_color: String,
}

impl Style for EditableBorderRadiusSelectorStyle_hover {
    fn create() -> Self {
        append_to_string!(
            Self {
                append: ":hover",
                background_color: "black !important",
            }
        )
    }
}

#[allow(non_snake_case)]
#[derive(Reflect)]
struct EditableBorderRadiusSelectorStyle {
    position: String,
    width: String,
    height: String,
    top: String,
    left: String,
    right: String,
    bottom: String,
    background_image: String,
    background_color: String,
    border_radius: String,
}

impl Style for EditableBorderRadiusSelectorStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                width: "10px",
                height: "10px",
                top: "initial",
                left: "initial",
                right: "initial",
                bottom: "initial",
                background_image: "none",
                background_color: "gray",
                border_radius: "100%",
            }
        )
    }
}

pub struct EditableBorderRadiusSelector {
    style: EditableBorderRadiusSelectorStyle
}

pub enum Msg {}

#[derive(PartialEq)]
pub enum Positions {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub position: Positions,
    pub onmousedown: Callback<MouseEvent>,
}

impl Component for EditableBorderRadiusSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {

        let mut top = "".to_string();
        let mut left = "".to_string();
        let mut right = "".to_string();
        let mut bottom = "".to_string();

        let distance = "20%";
        let max_distance = "50px";

        match ctx.props().position {
            Positions::TopLeft => {
                left = format!("clamp(0px, {}, {})", distance, max_distance);
                top = left.clone();
            }
            Positions::TopRight => {
                right = format!("clamp(0px, {}, {})", distance, max_distance);
                top = right.clone();
            }
            Positions::BottomLeft => {
                left = format!("clamp(0px, {}, {})", distance, max_distance);
                bottom = left.clone();
            }
            Positions::BottomRight => {
                right = format!("clamp(0px, {}, {})", distance, max_distance);
                bottom = right.clone();
            }
        }

        let mut style = EditableBorderRadiusSelectorStyle::create();
        style.left = left;
        style.top = top;
        style.right = right;
        style.bottom = bottom;

        Self {
            style: style,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let window = window().expect("No global `window` found");
        let document = window.document().expect("couldn't get `document");

        let hover_class = EditableBorderRadiusSelectorStyle_hover::create().as_class(&document);

        html! {
            <div class = { hover_class.unwrap() }
            style = { self.style.inline() } 
            onmousedown = { &ctx.props().onmousedown }>
            </div>
        }
    }
}