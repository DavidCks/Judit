use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

use crate::App;

#[allow(non_snake_case)]
#[derive(Reflect, Clone)]
struct ToolbarStyle {
    position: String,
    left: String,
    top: String,
    background_color: String,
    width: String,
    height: String,
    padding_top: String,
}

impl Style for ToolbarStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "fixed",
                left: "0px",
                top: "0px",
                background_color: "lightgray",
                width: "40px",
                height: "100%",
                padding_top: "5px",
            }
        )
    }
}

pub struct Toolbar {
   style: ToolbarStyle
}

pub enum Msg {
    AddElement,
}

impl Component for Toolbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: ToolbarStyle::create(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddElement => {
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        
        let li_style = "line-height: 0px; padding: 2.5px 0 2.5px 0;";

        let jrole_toolbar_cursor = "jrole=\"Judit_Toolbar_Cursor\"";
        let jrole_toolbar_addelement = "jrole=\"Judit_Toolbar_AddElement\"";
        let jrole_toolbar_addtext = "jrole=\"Judit_Toolbar_AddText\"";
        let jrole_toolbar_addimage = "jrole=\"Judit_Toolbar_AddImage\"";
        let jrole_toolbar_divider = "jrole=\"Judit_Toolbar_Divider\"";
        let jrole_toolbar_addcomponent = "jrole=\"Judit_Toolbar_AddComponent\"";

        html! {
            <ul style={ self.style.clone().inline() }>
                // Cursor
                <li style={ li_style } jrole={ jrole_toolbar_cursor }>
                    <svg width={ self.style.width.clone() } height={ self.style.width.clone() } jrole={ jrole_toolbar_cursor } stroke_width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path jrole={ jrole_toolbar_cursor } clip_rule="evenodd" d="M19.503 9.97c1.204.489 1.112 2.224-.137 2.583l-6.306 1.813-2.88 5.895c-.57 1.168-2.295.957-2.568-.314L4.677 6.257A1.369 1.369 0 016.53 4.7L19.503 9.97z" stroke="#000000" stroke_width="1.5"></path>
                    </svg>
                </li>
                // Add Element
                <li style={ li_style } jrole={ jrole_toolbar_addelement }>
                    <svg width={ self.style.width.clone() } height={ self.style.width.clone() } jrole={ jrole_toolbar_addelement } stroke_width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path jrole={ jrole_toolbar_addelement } d="M21 3.6v16.8a.6.6 0 01-.6.6H3.6a.6.6 0 01-.6-.6V3.6a.6.6 0 01.6-.6h16.8a.6.6 0 01.6.6z" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round"></path>
                    </svg>
                </li>
                // Add Text
                <li style={ li_style } jrole={ jrole_toolbar_addtext }>
                    <svg width={ self.style.width.clone() } height={ self.style.width.clone() } jrole={ jrole_toolbar_addtext } stroke_width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path jrole={ jrole_toolbar_addtext } d="M19 7V5H5v2M12 5v14m0 0h-2m2 0h2" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round"></path>
                    </svg>
                </li>
                // Add Image
                <li style={ li_style } jrole={ jrole_toolbar_addimage }>
                    <svg width={ self.style.width.clone() } height={ self.style.width.clone() } jrole={ jrole_toolbar_addimage } stroke_width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path jrole={ jrole_toolbar_addimage } d="M21 3.6v16.8a.6.6 0 01-.6.6H3.6a.6.6 0 01-.6-.6V3.6a.6.6 0 01.6-.6h16.8a.6.6 0 01.6.6z" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke-linejoin="round"></path><path d="M3 16l7-3 11 5M16 10a2 2 0 110-4 2 2 0 010 4z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                </li>
                // Divider
                <li style={ li_style } jrole={ jrole_toolbar_divider }>
                    <svg width={ self.style.width.clone() } height={ format!("{}px", self.style.width.clone().try_to_f64().unwrap() / 4_f64) } jrole={ jrole_toolbar_divider } stroke_width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path jrole={ jrole_toolbar_divider } d="M3 12L21 12" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round"></path>
                    </svg>
                </li>
                // Add Custom Component
                <li style={ li_style } jrole={ jrole_toolbar_addcomponent }>
                    <svg width={ self.style.width.clone() } height={ self.style.width.clone() } jrole={ jrole_toolbar_addcomponent } stroke_width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path jrole={ jrole_toolbar_addcomponent } d="M18.819 13.329l-5.324 5.99a2 2 0 01-2.99 0l-5.324-5.99a2 2 0 010-2.658l5.324-5.99a2 2 0 012.99 0l5.324 5.99a2 2 0 010 2.658zM9 12h3m3 0h-3m0 0V9m0 3v3" stroke="#000000" stroke_width="1.5" stroke_linecap="round" stroke_linejoin="round"></path>
                    </svg>
                </li>
            </ul>
        }
    }
}