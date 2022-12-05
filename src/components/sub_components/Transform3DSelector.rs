use rusty_css::*;
use yew::prelude::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct Transform {
    rotateX: String,
    rotateY: String,
    rotateZ: String,
}

#[allow(non_snake_case)]
#[derive(Reflect)]
struct Transform3DSelectorStyle {
    position: String,
    top: String,
    left: String,
    transform_origin: String,
    transform_style: String,
    transform: Transform,
    margin: String,
}

impl Style for Transform3DSelectorStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                position: "absolute",
                top: "0px",
                left: "0px",
                transform_style: "preserve-3d",
                transform_origin: "50% 50%",
                transform: Transform { rotateX: "0deg", rotateY: "0deg", rotateZ: "0deg" },
                margin: "0px 0px 0px 0px",
            }
        )
    }
}

pub enum Msg {
}

pub struct Transform3DSelector {
    style: Transform3DSelectorStyle,
    radius: i16,
}

impl Component for Transform3DSelector {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        let radius = 40;
        let mut style = Transform3DSelectorStyle::create();

        style.top = format!("calc(50% - {}px)", radius);
        style.left = format!("calc(50% - {}px)", radius);

        Self {
            style: style,
            radius: radius,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let faning = 87; //deg
        let rotation_disc_base_style = "transform-origin: 50% 50%; position: absolute; opacity: .5;";

        // x-axis
        let x1_style = format!("{} transform: rotateY(90deg) rotateX({}deg);", rotation_disc_base_style, faning);
        let x2_style = format!("{} transform: rotateY(180deg) rotateX({}deg);", rotation_disc_base_style, faning);
        let x3_style = format!("{} transform: rotateY(270deg) rotateX({}deg);", rotation_disc_base_style, faning);
        let x4_style = format!("{} transform: rotateY(360deg) rotateX({}deg);", rotation_disc_base_style, faning);

        // y-axis
        let y1_style = format!("{} transform: rotateX(90deg) rotateY({}deg);", rotation_disc_base_style, faning);
        let y2_style = format!("{} transform: rotateX(180deg) rotateY({}deg);", rotation_disc_base_style, faning);
        let y3_style = format!("{} transform: rotateX(270deg) rotateY({}deg);", rotation_disc_base_style, faning);
        let y4_style = format!("{} transform: rotateX(360deg) rotateY({}deg);", rotation_disc_base_style, faning);

        // z-axis
        let z1_style = format!("{} transform: rotateZ(90deg) rotateY({}deg);", rotation_disc_base_style, faning - 90);
        let z2_style = format!("{} transform: rotateZ(180deg) rotateY({}deg);", rotation_disc_base_style, faning - 90);
        let z3_style = format!("{} transform: rotateZ(270deg) rotateY({}deg);", rotation_disc_base_style, faning - 90);
        let z4_style = format!("{} transform: rotateZ(360deg) rotateY({}deg);", rotation_disc_base_style, faning - 90);

        let rotate_x_jrole = "Judit_Transform3DSelector_Rotate_X";
        let rotate_y_jrole = "Judit_Transform3DSelector_Rotate_Y";
        let rotate_z_jrole = "Judit_Transform3DSelector_Rotate_Z";

        html! {
            <div style={ format!("{} width: {r2}px; height: {r2}px", self.style.inline(), r2 = self.radius*2 ) }>
                // x-axis spanning disc
                <svg style={x1_style} jrole="Judit_EditableElement" axis="X" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_y_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="red" />
                </svg>
                <svg style={x2_style} jrole="Judit_EditableElement" axis="X" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_y_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="red" />
                </svg>
                <svg style={x3_style} jrole="Judit_EditableElement" axis="X" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_y_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="red" />
                </svg>
                <svg style={x4_style} jrole="Judit_EditableElement" axis="X" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_y_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="red" />
                </svg>

                // y-axis spanning disc
                <svg style={y1_style} jrole="Judit_EditableElement" axis="Y" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_x_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="blue" />
                </svg>
                <svg style={y2_style} jrole="Judit_EditableElement" axis="Y" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_x_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="blue" />
                </svg>
                <svg style={y3_style} jrole="Judit_EditableElement" axis="Y" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_x_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="blue" />
                </svg>
                <svg style={y4_style} jrole="Judit_EditableElement" axis="Y" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_x_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="blue" />
                </svg>

                // z-axis spanning disc
                <svg style={z1_style} jrole="Judit_EditableElement" axis="Z" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_z_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="green" />
                </svg>
                <svg style={z2_style} jrole="Judit_EditableElement" axis="Z" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_z_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="green" />
                </svg>
                <svg style={z3_style} jrole="Judit_EditableElement" axis="Z" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_z_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="green" />
                </svg>
                <svg style={z4_style} jrole="Judit_EditableElement" axis="Z" height={ (self.radius * 2).to_string() } width={ (self.radius * 2).to_string() }>
                    <circle jrole={ rotate_z_jrole } cx={ self.radius.to_string() } cy={ self.radius.to_string() } r={ self.radius.to_string() } fill="green" />
                </svg>

            </div> 
        }
    }
}