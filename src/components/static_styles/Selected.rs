use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect)]
pub struct Selected {
    pub box_shadow: String,
}

impl Style for Selected {
    fn create() -> Self {
        append_to_string!( 
            Self {
                box_shadow: "0px 0px 5px 1px cornflowerblue",
            }
        )
    }
}
