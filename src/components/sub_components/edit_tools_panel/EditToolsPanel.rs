use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

use crate::Transform;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct EditToolsPanelStyle {
    position: String,
    left: String,
    top: String,
    width: String,
    writing_mode: String,
    direction: String,
}

impl Style for EditToolsPanelStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                left: "calc(100% + 4px)",
                top: "0px",
                width: "fit-content",
                writing_mode: "horizontal-tb",
                direction: "ltr",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct EditToolsPanelProps {
    #[prop_or_default]
    pub children: Children,
    pub parent_transform: Transform,
}

pub struct EditToolsPanel {
   style: EditToolsPanelStyle
}

pub enum Msg {
}

impl Component for EditToolsPanel {
    type Message = Msg;
    type Properties = EditToolsPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: EditToolsPanelStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let rx = format!("rotateX({}deg)", ctx.props().parent_transform.rotateX.try_to_f64().unwrap() * -1_f64); 
        let ry = format!("rotateY({}deg)", ctx.props().parent_transform.rotateY.try_to_f64().unwrap() * -1_f64);
        let rz = format!("rotateZ({}deg)", ctx.props().parent_transform.rotateZ.try_to_f64().unwrap() * -1_f64);

        html! {
            <div style={ format!("{init_style} transform: {} {} {}", rz, ry, rx, init_style = self.style.inline() )} >
                { for ctx.props().children.iter() }
            </div>
            
        }
    }
}