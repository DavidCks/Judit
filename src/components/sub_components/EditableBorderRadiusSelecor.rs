//use log::info;

use rusty_css::*;
use yew::{prelude::*};
use web_sys::{ window };
use bevy_reflect::{ Reflect };
use std::str::FromStr;
use strum_macros::EnumString;
use append_to_string::*;

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Reflect)]
struct EditableBorderRadiusSelectorStyle_Hover {
    append: String,
    background_color: String,
}

impl Style for EditableBorderRadiusSelectorStyle_Hover {
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
    border_width: String,
    border_color: String,
    border_style: String,
    box_sizing: String,
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
                border_radius: "",
                border_width: "",
                border_color: "",
                border_style: "solid",
                box_sizing: "border-box",
            }
        )
    }
}

pub struct EditableBorderRadiusSelector {
    style: EditableBorderRadiusSelectorStyle
}

pub enum Msg {}

#[derive(PartialEq, EnumString, Debug)]
pub enum Positions {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[derive(PartialEq, Clone)]
pub struct BorderSelectorStyle {
    pub border_width: String,
    pub border_color: String,
    pub border_radius: String,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub position: Positions,
    pub border: BorderSelectorStyle,
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

        style.border_color = ctx.props().border.border_color.clone();
        style.border_width = ctx.props().border.border_width.clone();
        style.border_radius = ctx.props().border.border_radius.clone();

        Self {
            style: style,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let window = window().expect("No global `window` found");
        let document = window.document().expect("couldn't get `document");

        let hover_class = EditableBorderRadiusSelectorStyle_Hover::create().as_class(&document);

        html! {
            <div 
            position = { format!("{:?}", ctx.props().position) }
            id = {"rusty-css_EditableBorderRadiusSelector"}
            class = { hover_class.unwrap() }
            style = { self.style.inline() }>
            </div>
        }
    }
}