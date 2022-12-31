use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct BorderColorButtonStyle {
    position: String,
    background_color: String,
    width: String,
    height: String,
    border: String,
    border_radius: String,
    appearance: String,
    padding: String,
    margin: String,
}

impl Style for BorderColorButtonStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "initial",
                background_color: "",
                width: "18px",
                height: "18px",
                border: "none",
                border_radius: "5px",
                appearance: "none",
                padding: "0px",
                margin: "3px",
            }
        )
    }
}

pub struct BorderColorButton {
   style: BorderColorButtonStyle
}

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct BorderColorButtonProps {
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or("#3f3f3f".to_string())]
    pub background_color: String,
}

impl Component for BorderColorButton {
    type Message = Msg;
    type Properties = BorderColorButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut style = BorderColorButtonStyle::create();
        style.background_color = ctx.props().background_color.clone();
        Self {
            style: style,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.style.background_color = ctx.props().background_color.clone();
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style={ self.style.inline() } >
                <input onchange={ ctx.props().onchange.clone() } style="opacity: 0; width: 100%; height: 100%" type="color"/>
            </div>
        }
    }
}