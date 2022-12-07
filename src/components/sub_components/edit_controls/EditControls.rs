use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

use crate::components::EditableElement::Transform;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct EditControlsStyle {
    position: String,
    width: String,
    height: String,
    top: String,
    left: String,
    display: String,
    align_items: String,
    justify_content: String,
}

impl Style for EditControlsStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                width: "auto",
                height: "30px",
                top: "calc(100% + 2px)",
                left: "0px",
                display: "flex",
                align_items: "flex-start",
                justify_content: "left",
            }
        )
    }
}

pub struct EditControls {
   style: EditControlsStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct EditControlsProps {
    pub parent_transform: Transform,
    #[prop_or_default]
    pub children: Children,
}

impl Component for EditControls {
    type Message = Msg;
    type Properties = EditControlsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: EditControlsStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let rx = format!("rotateX({}deg)", ctx.props().parent_transform.rotateX.try_to_f64().unwrap() * -1_f64); 
        let ry = format!("rotateY({}deg)", ctx.props().parent_transform.rotateY.try_to_f64().unwrap() * -1_f64);
        let rz = format!("rotateZ({}deg)", ctx.props().parent_transform.rotateZ.try_to_f64().unwrap() * -1_f64);

        html! {
            <div jrole="Judit_EditControls" style={ format!("{init_style} transform: {} {} {}", rz, ry, rx, init_style = self.style.inline()) }>
                { for ctx.props().children.iter() }
            </div>
            
        }
    }
}