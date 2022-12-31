use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderEditPanelStyle {
    display: String,
    grid_template_columns: String,
    gap: String,
    align_items: String,

    position: String,
    border_radius: String,
}

impl Style for BorderEditPanelStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                display: "grid",
                grid_template_columns: "1fr 1fr 1fr 1fr",
                gap: "0px 0px",
                align_items: "center",

                position: "initial",
                border_radius: "10px",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct BorderEditPanelProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct BorderEditPanel {
   style: BorderEditPanelStyle
}

pub enum Msg {
}

impl Component for BorderEditPanel {
    type Message = Msg;
    type Properties = BorderEditPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: BorderEditPanelStyle::create(),
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