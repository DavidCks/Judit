use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

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

#[derive(Properties, PartialEq)]
pub struct EditControlsProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct EditControls {
   style: EditControlsStyle
}

pub enum Msg {
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
        html! {
            <div jrole="Judit_EditControls" style={ self.style.inline() } >
                { for ctx.props().children.iter() }
            </div>
            
        }
    }
}