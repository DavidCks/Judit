use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::window;

// external styles
use crate::components::static_styles::Selected::Selected as SelectedStyle;

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Reflect)]
struct FontPickerStyle_fokus {
    append: String,
    border_width: String,
    border_color: String,
    border_style: String,    
}

impl Style for FontPickerStyle_fokus {
    fn create() -> Self {

        let selected_style = SelectedStyle::create();

        append_to_string!(
            Self {
                append: ":focus",
                border_width: selected_style.border_width.clone(),
                border_color: selected_style.border_color,
                border_style: "solid !important",
            }
        )
    }
}

#[allow(non_snake_case)]
#[derive(Reflect)]
struct FontPickerStyle {
    position: String,
    grid_column: String,
    border_radius: String,
    border_style: String,
    border_width: String,
    background_color: String,
    font_size: String,
    height: String,
    display: String,
    justify_content: String,
    align_content: String,
    align_items: String,
}

impl Style for FontPickerStyle {
    fn create() -> Self {

        let selected_style = SelectedStyle::create();

        append_to_string!(
            Self {
                display: "flex",
                justify_content: "center",
                align_items: "center",
                align_content: "center",
                position: "initial",
                grid_column: "span 4",
                border_style: "solid",
                border_width: selected_style.border_width.clone(),
                border_radius: "10px",
                background_color: "#EEEEEE",
                font_size: "1rem",
                height: format!("calc(100% - {} * 2)", &selected_style.border_width),
            }
        )
    }
}

pub struct FontPicker {
   style: FontPickerStyle,
   focus_style: FontPickerStyle_fokus,
   fonts: Vec<Html>,
}

#[derive(Properties, PartialEq)]
pub struct FontPickerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

pub enum Msg {
}

impl Component for FontPicker {
    type Message = Msg;
    type Properties = FontPickerProps;

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            focus_style: FontPickerStyle_fokus::create(), 
            style: FontPickerStyle::create(), 
            fonts: vec![
                html!(<option style="font-family: Arial, sans-serif" value="Arial, sans-serif">{ "Arial (sans-serif)" }</option>),
                html!(<option style="Verdana, sans-serif" value="Verdana, sans-serif">{"Verdana (sans-serif)"}</option>),
                html!(<option style="Tahoma, sans-serif" value="Tahoma, sans-serif">{"Tahoma (sans-serif)"}</option>),
                html!(<option style="Trebuchet MS, sans-serif" value="Trebuchet MS, sans-serif">{"Trebuchet MS (sans-serif)"}</option>),    
                html!(<option style="Times New Roman, serif" value="Times New Roman, serif">{"Times New Roman (serif)"}</option>),    
                html!(<option style="Georgia, serif" value="Georgia, serif">{"Georgia (serif)"}</option>),
                html!(<option style="Garamond, serif" value="Garamond, serif">{"Garamond (serif)"}</option>),    
                html!(<option style="Courier New, monospace" value="Courier New, monospace">{"Courier New (monospace)"}</option>),    
                html!(<option style="Brush Script MT, cursive" value="Brush Script MT, cursive">{"Brush Script MT (cursive)"}</option>),    
            ],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let window = window().expect("No global `window` found");
        let document = window.document().expect("couldn't get `document");

        let standard_class = self.style.as_class(&document).unwrap();
        let focus_class =  self.focus_style.as_class(&document).unwrap();
        
        html! {
            <select onchange={ctx.props().onchange.clone()} style={ self.style.inline() } class={ format!("{} {}", focus_class, standard_class) } >
                { for self.fonts.clone() }
            </select>
        }
    }
}