use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
pub struct DropzoneHoverStyle {
    append: String,
    background_color: String,
}

impl Style for DropzoneHoverStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                append: ":hover",
                background_color: "red",
            }
        )
    }
}
