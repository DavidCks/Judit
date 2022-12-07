use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

use crate::components::EditableElement::Transform;

#[allow(non_snake_case)]
#[derive(Reflect, Clone)]
struct DeleteButtonStyle {
    position: String,
    width: String,
    height: String,
    right: String,
    top: String,
    transform: Transform,
}

impl Style for DeleteButtonStyle {
    fn create() -> Self {

        let width = "30px".to_owned();

        append_to_string!(
            Self {
                position: "absolute",
                width: width.clone(),
                height: width.clone(),
                right: "-5px",
                top: format!("calc(-{} - 2px)", width),
                transform: Transform { 
                    skewX: "0deg",
                    skewY: "0deg",
                    translateX: "0px",
                    rotateX: "0deg",
                    rotateY: "0deg",
                    rotateZ: "0deg",
                },
            }
        )
    }
}

pub struct DeleteButton {
   style: DeleteButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProps {
    pub parent_transform: Transform,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for DeleteButton {
    type Message = Msg;
    type Properties = DeleteButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {        
        Self {
            style: DeleteButtonStyle::create(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut style = self.style.clone();
        // style.transform.rotateX = format!("{}deg", ctx.props().parent_transform.rotateX.try_to_f64().unwrap() * -1_f64); 
        // style.transform.rotateY = format!("{}deg", ctx.props().parent_transform.rotateY.try_to_f64().unwrap() * -1_f64); 
        // style.transform.rotateZ = format!("{}deg", ctx.props().parent_transform.rotateZ.try_to_f64().unwrap() * -1_f64); 

        html! {
            <svg style={ style.inline() } onclick={ ctx.props().onclick.clone() } jrole="Judit_DeleteButton" width="24px" height="24px" stroke_width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path jrole="Judit_DeleteButton" d="M9.879 14.121L12 12m2.121-2.121L12 12m0 0L9.879 9.879M12 12l2.121 2.121M21 3.6v16.8a.6.6 0 01-.6.6H3.6a.6.6 0 01-.6-.6V3.6a.6.6 0 01.6-.6h16.8a.6.6 0 01.6.6z" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round"></path>
            </svg>
        }
    }
}