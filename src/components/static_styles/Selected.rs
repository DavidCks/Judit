use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect)]
pub struct Selected {
    border_width: String,
    border_color: String,
    border_style: String,
}

impl Style for Selected {
    fn create() -> Self {
        append_to_string!( 
            Self {
                border_width: "2px",
                border_color: "cornflowerblue",
                border_style: "dashed",
            }
        )
    }
}
