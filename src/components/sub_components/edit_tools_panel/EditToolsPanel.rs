use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct EditToolsPanelStyle {
    position: String,
    left: String,
    width: String,
}

impl Style for EditToolsPanelStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "relative",
                left: "calc(100% + 4px)",
                width: "fit-content",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct EditToolsPanelProps {
    #[prop_or_default]
    pub children: Children,
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
        html! {
            <div style={ self.style.inline() } >
                { for ctx.props().children.iter() }
            </div>
            
        }
    }
}