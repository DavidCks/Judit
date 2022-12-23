use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect)]
pub struct Selected {
    pub border_width: String,
    pub border_color: String,
    pub border_style: String,
    pub z_index: String,
}

impl Style for Selected {
    fn create() -> Self {
        append_to_string!( 
            Self {
                border_width: "2px",
                border_color: "cornflowerblue",
                border_style: "dashed",
                z_index: "1",
            }
        )
    }
}
