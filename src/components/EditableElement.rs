use log::{ info };
use wasm_bindgen::JsCast;
use std::ops::Deref;
use std::str::FromStr;
//use log::info;
use yew::{prelude::*};
use yew::html::{Scope};
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ MouseEvent, Element, HtmlSelectElement, Document, window };
use rusty_css::*;
use super::super::Msg as PMsg;

use super::sub_components::EditableBorderRadiusSelecor::EditableBorderRadiusSelector as EditableBorderRadiusSelector;
use super::sub_components::EditableBorderRadiusSelecor::Positions as ebrsPositions;
use super::sub_components::EditableBorderRadiusSelecor::BorderSelectorStyle;

// 3D edit ball
use super::sub_components::Transform3DSelector::Transform3DSelector as Transform3DSelector;

// Alyways visible Buttons surrounding EditableElement
use super::sub_components::DeleteButton::DeleteButton as DeleteButton;
use super::sub_components::edit_controls::EditControls::EditControls as EditControls;
    use super::sub_components::edit_controls::Transform3DToggle::Transform3DToggle as Transform3DToggle;
    use super::sub_components::edit_controls::Transform2DToggle::Transform2DToggle as Transform2DToggle;

// Optionally visible Buttons surrounding EditableElement
use super::sub_components::edit_tools_panel::EditToolsPanel::EditToolsPanel as EditToolsPanel;
    use super::sub_components::edit_tools_panel::TextEditPanel::TextEditPanel as TextEditPanel;
        use super::sub_components::edit_tools_panel::text_edit_buttons::AlignRightButton::AlignRightButton as AlignRightButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::AlignCenterButton::AlignCenterButton as AlignCenterButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::AlignJustifyButton::AlignJustifyButton as AlignJustifyButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::AlignLeftButton::AlignLeftButton as AlignLeftButton;

        use super::sub_components::edit_tools_panel::text_edit_buttons::StyleBoldButton::StyleBoldButton as StyleBoldButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::StyleItalicButton::StyleItalicButton as StyleItalicButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::StyleUnderlineButton::StyleUnderlineButton as StyleUnderlineButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::StyleSizeButton::StyleSizeButton as StyleSizeButton;

        use super::sub_components::edit_tools_panel::text_edit_buttons::SpacingLinesButton::SpacingLinesButton as SpacingLinesButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::SpacingWordsButton::SpacingWordsButton as SpacingWordsButton;
        use super::sub_components::edit_tools_panel::text_edit_buttons::SpacingLettersButton::SpacingLettersButton as SpacingLettersButton;

        use super::sub_components::edit_tools_panel::text_edit_buttons::LeftAlignedHorizontalText::LeftAlignedHorizontalText as LeftAlignedHorizontalText;
        use super::sub_components::edit_tools_panel::text_edit_buttons::RightAlignedVerticalText::RightAlignedVerticalText as RightAlignedVerticalText;
        use super::sub_components::edit_tools_panel::text_edit_buttons::LeftAlignedVerticalText::LeftAlignedVerticalText as LeftAlignedVerticalText;

// Drag and Drop
use super::sub_components::drop_zones::DropzoneLeft::DropzoneLeft as DropzoneLeft;
use super::sub_components::drop_zones::DropzoneTop::DropzoneTop as DropzoneTop;
use super::sub_components::drop_zones::DropzoneRight::DropzoneRight as DropzoneRight;
use super::sub_components::drop_zones::DropzoneBottom::DropzoneBottom as DropzoneBottom;
use super::sub_components::drop_zones::DropzoneBottomLeft::DropzoneBottomLeft as DropzoneBottomLeft;
use super::sub_components::drop_zones::DropzoneBottomRight::DropzoneBottomRight as DropzoneBottomRight;
use super::sub_components::drop_zones::DropzoneTopLeft::DropzoneTopLeft as DropzoneTopLeft;
use super::sub_components::drop_zones::DropzoneTopRight::DropzoneTopRight as DropzoneTopRight;
use super::sub_components::drop_zones::DropzoneNoAlign::DropzoneNoAlign as DropzoneNoAlign;

// font picker
use super::sub_components::edit_tools_panel::FontPicker::FontPicker as FontPicker;

// external styles
use super::static_styles::Selected::Selected as SelectedStyle;

#[allow(non_camel_case_types)]
#[derive(Reflect, Clone)]
struct Judit_MakeDropzone {
    z_index: String,
}

impl Style for Judit_MakeDropzone {
    fn create() -> Self {
        append_to_string!( 
            Self {
                z_index: "2",
            }
        )
    }
}

#[allow(non_snake_case)]
#[derive(Reflect, PartialEq, Clone)]
pub struct Transform {
    pub skewX: String,
    pub skewY: String,
    pub translateX: String,
    pub rotateX: String,
    pub rotateY: String,
    pub rotateZ: String,
}

#[allow(non_snake_case)]
#[derive(Reflect, Clone, PartialEq)]
pub struct ComponentStyle {
    position: String,
    box_sizing: String,

    top: String,
    left: String,
    width: String,
    height: String,

    background_color: String,

    transform_origin: String,
    transform_style: String,
    transform: Transform,

    border_top_left_radius: String,
    border_top_right_radius: String,
    border_bottom_left_radius: String,
    border_bottom_right_radius: String,
    border_color: String,
    border_style: String,
    border_width: String,

    // text stuff
    text_align: String,
    writing_mode: String,
    text_orientation: String,
    text_decoration_line: String,
    font_weight: String,
    font_style: String,
    font_size: String,
    letter_spacing: String,
    word_spacing: String,
    line_height: String,
    font_family: String,
}

impl Style for ComponentStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                position: "absolute",
                box_sizing: "border-box",
                top: "10px",
                left: "50px",
                width: "200px",
                height: "200px",
                background_color: "#EEEEEE",
                transform_origin: "50% 50%",
                transform_style: "preserve-3d",
                transform: Transform { 
                    skewX: "0deg",
                    skewY: "0deg",
                    translateX: "0px",
                    rotateX: "0deg",
                    rotateY: "0deg",
                    rotateZ: "0deg",
                },
                border_top_left_radius: "10px",
                border_top_right_radius: "10px",
                border_bottom_left_radius: "10px",
                border_bottom_right_radius: "10px",
                border_color: "#3f3f3f",
                border_style: "solid",
                border_width: "2px",
                // writing stuff
                text_align: "left",
                writing_mode: "horizontal-tb",
                text_orientation: "upright",
                text_decoration_line: "none", // for underline
                font_weight: "normal", // for bold
                font_style: "normal", //for italic
                font_size: "1rem",
                letter_spacing: "0em",
                word_spacing: "0.25em",
                line_height: "1",
                font_family: "Arial, sans-serif",
            }
        )
    }
}

#[derive(PartialEq, Clone)]
pub enum JTypes {
    Div,
    Text,
    Image
}

#[derive(Clone)]
pub enum EditStates {
    None,
    Move,
    Resize,
    BorderRadius,
    Edit3D,
    Text,
}

#[derive(Clone)]
// Component Fuctions
pub enum Msg {
    // Drag and Drop / resize / border radius
    StartEditingWithCursor(MouseEvent),
    StopEditingWithCursor(MouseEvent),
    
    // Receive Parents mouse move event
    ReceiveCursorMove(MouseEvent),

    Select(MouseEvent),
    Deselect,
    Delete,

    Transform3DToggle,
    Transform2DToggle,

    //////////////////////
    // Edit Panel Stuff //
    //////////////////////
    
    // Text Edit Panel
    AlignTextLeft,
    AlignTextCenter,
    AlignTextJustify,
    AlignTextRight,

    StyleTextBold,
    StyleTextItalic,
    StyleTextUnderline,
    StyleTextSize,

    SpacingLetters,
    SpacingWords,
    SpacingLines,

    TextDirectionLRHorizontal,
    TextDirectionRLVertical,
    TextDirectionLRVertical,

    PickFont(Event),

    // drop into element as child
    EnableDropzones,
    DisableDropzones,
    DisplayDropzones(MouseEvent),
    HideDropzones,

    // - with alignment
    DropInBottom,

    Noop,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or(JTypes::Div)]
    pub jtype: JTypes,

    #[prop_or_default]
    pub tag: String,

    #[prop_or(ComponentStyle::create())]
    pub init_style: ComponentStyle,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub jdepth: u32, // essentially amounut of parent EditableElements
}

#[derive(Clone)]
pub struct EditableElement {

    parent_link: Scope<super::super::App>,
    document: Document,
    children: Vec<EditableElement>,

    style: ComponentStyle,
    border_selector_style_topleft: BorderSelectorStyle,
    border_selector_style_topright: BorderSelectorStyle,
    border_selector_style_bottomleft: BorderSelectorStyle,
    border_selector_style_bottomright: BorderSelectorStyle,

    // contols
    editing_state: EditStates,

    is_movable: bool, 

    is_resizeable: bool,
    is_resizeable_left: bool,
    is_resizeable_right: bool,
    is_resizeable_top: bool,
    is_resizeable_bottom: bool,

    is_eidting_radius: bool,
    is_eidting_radius_topleft: bool,
    is_eidting_radius_topright: bool,
    is_eidting_radius_bottomleft: bool,
    is_eidting_radius_bottomright: bool,

    is_editing_3d: bool,
    is_editing_3d_rotate_x: bool,
    is_editing_3d_rotate_y: bool,
    is_editing_3d_rotate_z: bool,
    
    // text edits relative to mouse
    is_editing_text_size: bool,
    is_editing_text_spacing_letters: bool,
    is_editing_text_spacing_lines: bool,
    is_editing_text_spacing_words: bool,

    // dragging into and out of an element
    is_dragging_in: bool,

    is_selected: bool,
    render: bool,

    // state
    previous_mouse_x: Option<i32>,
    previous_mouse_y: Option<i32>,
}

impl EditableElement {
    fn event_jdepth(&self, e: MouseEvent) -> Option<u32> {
        let target = e.target_dyn_into::<Element>().unwrap();
        let element_jdepth = target.get_attribute("jdepth").unwrap_or("none".to_string());

        if element_jdepth != "none".to_string() { 
            return Some(element_jdepth.parse::<u32>().unwrap());
        } else {
            return None;
        }
    }
}

impl Component for EditableElement {
    type Properties = Props;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {

        let mut parent_any_link = ctx.link().get_parent().expect("No Parent found").clone();
        let mut parent_link = parent_any_link.try_downcast::<super::super::App>();
        while parent_link.is_none() {
            parent_any_link = parent_any_link.get_parent().expect("Error in recursive parent search").clone();
            parent_link = parent_any_link.try_downcast::<super::super::App>();
        }

        let initial_border_selector_style = append_to_string!( 
            BorderSelectorStyle { 
                border_width: "3px", 
                border_color: "#3f3f3f", 
                border_radius: "100%" 
            }
        );

        Self {
            parent_link: parent_link.unwrap(),
            document: window().unwrap().document().unwrap(),

            children: Vec::new(),

            style: ctx.props().init_style.clone(),
            border_selector_style_topleft: initial_border_selector_style.clone(),
            border_selector_style_topright: initial_border_selector_style.clone(),
            border_selector_style_bottomleft: initial_border_selector_style.clone(),
            border_selector_style_bottomright: initial_border_selector_style.clone(),

            previous_mouse_x: None,
            previous_mouse_y: None,

            editing_state: EditStates::None,

            is_movable: false,

            is_resizeable: false,
            is_resizeable_left: false,
            is_resizeable_right: false,
            is_resizeable_top: false,
            is_resizeable_bottom: false,

            is_eidting_radius: false,
            is_eidting_radius_bottomleft: false,
            is_eidting_radius_bottomright: false,
            is_eidting_radius_topleft: false,
            is_eidting_radius_topright: false,

            is_editing_3d: false,
            is_editing_3d_rotate_x: false,
            is_editing_3d_rotate_y: false,
            is_editing_3d_rotate_z: false,

            is_editing_text_size: false,
            is_editing_text_spacing_letters: false,
            is_editing_text_spacing_lines: false,
            is_editing_text_spacing_words: false,

            is_dragging_in: false,

            is_selected: false,
            render: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(e) => {

                let element_jdepth = self.event_jdepth(e);

                if element_jdepth.is_none() || 
                   ctx.props().jdepth != element_jdepth.unwrap() { return false } // prevent firing on the wrong element
                if self.is_selected { return false }

                // Send this comoponents instrance context to the parent.
                let child_link = ctx.link().clone();
                self.parent_link.send_message( PMsg::ReceiveSelectedChildLink( child_link ));
                
                self.is_selected = true;

                return true;
            }
            Msg::Deselect => {
                self.is_selected = false;
                self.editing_state = EditStates::None;

                // reset previous mouse position
                self.previous_mouse_x = None;
                self.previous_mouse_y = None;
                true
            }
            Msg::Delete => {
                self.render = false;
                true
            }
            // 3D Transform enable / disable
            Msg::Transform3DToggle => {
                self.is_editing_3d = true;
                true
            }
            Msg::Transform2DToggle => {
                self.is_editing_3d = false;
                true
            }
            Msg::StartEditingWithCursor(e) => {
                ///////////////////////////////////////////////////////////////////////////
                // edit elements width / height or move element based on cursor position //
                ///////////////////////////////////////////////////////////////////////////

                // select if not already selected
                if !self.is_selected {
                    ctx.link().send_message(Msg::Select(e.clone()));
                }

                let target = e.target_dyn_into::<Element>().unwrap();

                match target.get_attribute("jrole").unwrap_or_default().as_str() {
                    // start editing border radius
                    "Judit_EditableBorderRadiusSelector" => {
                        // endable editing the radius
                        self.is_eidting_radius = true;
                        self.editing_state = EditStates::BorderRadius;

                        match ebrsPositions::from_str( &target.get_attribute("position").unwrap() ).unwrap() {
                            ebrsPositions::TopLeft => {
                                self.is_eidting_radius_topleft = true;
                            }
                            ebrsPositions::TopRight => {
                                self.is_eidting_radius_topright = true;
                            }
                            ebrsPositions::BottomLeft => {
                                self.is_eidting_radius_bottomleft = true;
                            }
                            ebrsPositions::BottomRight => {
                                self.is_eidting_radius_bottomright = true;
                            }
                        }
                    }
                    "Judit_EditableElement" => {

                        // determine wether the border has been clicked or the box
                        let resize_range = 10;

                        // horizontal resize
                        let offset_x = e.offset_x();
                        if offset_x < resize_range { // left side
                            self.is_resizeable = true;
                            self.editing_state = EditStates::Resize;
                            self.is_resizeable_left = true;
                        } else {
                            let width = self.style.width.try_to_f64().unwrap();
                            if width - <i32 as Into<f64>>::into(resize_range) < offset_x.into() { // right side
                                self.is_resizeable = true;
                                self.editing_state = EditStates::Resize;
                                self.is_resizeable_right = true;
                            }
                        }

                        let offset_y = e.offset_y();
                        if offset_y < resize_range { // top side
                            self.is_resizeable = true;
                            self.editing_state = EditStates::Resize;
                            self.is_resizeable_top = true;
                        } else {
                            let height = self.style.height.try_to_f64().unwrap();
                            if height - <i32 as Into<f64>>::into(resize_range) < offset_y.into() { // bottom side
                                self.is_resizeable = true;
                                self.editing_state = EditStates::Resize;
                                self.is_resizeable_bottom = true;
                            }
                        }

                        // free movement
                        if !self.is_resizeable {
                            self.is_movable =  true;
                            self.editing_state = EditStates::Move;
                        }
                    }
                    // 3D Rotates
                    "Judit_Transform3DSelector_Rotate_X" => {
                        self.editing_state = EditStates::Edit3D;
                        self.is_editing_3d_rotate_x = true;
                    }
                    "Judit_Transform3DSelector_Rotate_Y" => {
                        self.editing_state = EditStates::Edit3D;
                        self.is_editing_3d_rotate_y = true;
                    }
                    "Judit_Transform3DSelector_Rotate_Z" => {
                        self.editing_state = EditStates::Edit3D;
                        self.is_editing_3d_rotate_z = true;
                    }
                    // Text Edits relative to mouse movement
                    "Judit_StyleSizeButton" => {
                        self.editing_state = EditStates::Text;
                        self.is_editing_text_size = true;
                    }
                    "Judit_SpacingLettersButton" => {
                        self.editing_state = EditStates::Text;
                        self.is_editing_text_spacing_letters = true;
                    }
                    "Judit_SpacingLinesButton" => {
                        self.editing_state = EditStates::Text;
                        self.is_editing_text_spacing_lines = true;
                    }
                    "Judit_SpacingWordsButton" => {
                        self.editing_state = EditStates::Text;
                        self.is_editing_text_spacing_words = true;
                    }
                    &_ => {
                        if let Some(jrole) = target.get_attribute("jrole") {
                            info!("unused jrole. jrole found: '{}'", jrole);
                        } else {
                            info!("No jrole attribute found.\n
                            only control elements that edit the selected element via mouse movenments (while the mouse is clicked) on the Canvas need a jrole.\n
                            while that might not be necessary for every element it is best to assign one just as a precaution for possible future implementations.");
                        }
                    }
                }

                false
            }
            Msg::StopEditingWithCursor(e) => {
                e.prevent_default();

                self.editing_state = EditStates::None;

                self.is_movable = false;

                self.is_resizeable = false;
                self.is_resizeable_bottom = false;
                self.is_resizeable_left = false;
                self.is_resizeable_right = false;
                self.is_resizeable_top = false;
                

                self.is_eidting_radius = false;
                self.is_eidting_radius_topleft = false;
                self.is_eidting_radius_topright = false;
                self.is_eidting_radius_bottomleft = false;
                self.is_eidting_radius_bottomright = false;

                self.is_editing_3d_rotate_x = false;
                self.is_editing_3d_rotate_y = false;
                self.is_editing_3d_rotate_z = false;

                self.is_editing_text_size = false;
                self.is_editing_text_spacing_letters = false;
                self.is_editing_text_spacing_lines = false;
                self.is_editing_text_spacing_words = false;

                false
            }
            Msg::ReceiveCursorMove(parent_e) => {

                if !self.is_selected {
                    return false;
                }

                // calculate offset from the previous mouse event
                let offset_x = parent_e.page_x() - self.previous_mouse_x.unwrap_or( parent_e.page_x() );
                let offset_y = parent_e.page_y() - self.previous_mouse_y.unwrap_or( parent_e.page_y() );

                match self.editing_state {
                    EditStates::Move => {
                        // calculate relative x and y positions for the offset in css
                        let relative_x: f64 = self.style.left.try_to_f64().unwrap() + f64::from(offset_x);
                        let relative_y: f64 = self.style.top.try_to_f64().unwrap() + f64::from(offset_y);

                        // assign new position and assign it to the style
                        self.style.left = format!("{}px", relative_x.trunc());
                        self.style.top = format!("{}px", relative_y.trunc());
                    }
                    EditStates::Resize => {
                        //vertical resize
                        if self.is_resizeable_bottom {
                            
                            let relative_height: f64 = self.style.height.try_to_f64().unwrap() + f64::from(offset_y);

                            if relative_height > 1_f64 {
                                self.style.height = format!("{}px", relative_height.trunc());
                            }

                        } else if self.is_resizeable_top {

                            let relative_y: f64 = self.style.top.try_to_f64().unwrap() + f64::from(offset_y);
                            let relative_height: f64 = self.style.height.try_to_f64().unwrap() - f64::from(offset_y);
                            
                            // prevent negative heights and movement after min. height is achieved
                            if relative_height > 1_f64 {
                                self.style.height = format!("{}px", relative_height.trunc());
                                self.style.top = format!("{}px", relative_y.trunc());
                            }
                        }

                        //horrizontal resize
                        if self.is_resizeable_right {
                            
                            let relative_width: f64 = self.style.width.try_to_f64().unwrap() + f64::from(offset_x);

                            // prevent negative widths
                            if relative_width > 1_f64 {
                                self.style.width = format!("{}px", relative_width.trunc());
                            }
                        } else if self.is_resizeable_left {

                            let relative_x: f64 = self.style.left.try_to_f64().unwrap() + f64::from(offset_x);
                            let relative_width: f64 = self.style.width.try_to_f64().unwrap() - f64::from(offset_x);

                            
                            self.style.left = format!("{}px", relative_x.trunc());
                            // prevent negative widths
                            if relative_width > 1_f64 {
                                self.style.width = format!("{}px", relative_width.trunc());
                            }
                        }
                    }
                    EditStates::BorderRadius => {
                        
                        if self.is_eidting_radius_topleft {
                            let relative_raidus: f64 = self.style.border_top_left_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < 50_f64 && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_top_left_radius = format!("{}%", relative_raidus.trunc());
                            }
                        }

                        if self.is_eidting_radius_topright {
                            let relative_raidus: f64 = self.style.border_top_right_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < 50_f64 && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_top_right_radius = format!("{}%", relative_raidus.trunc());
                            }
                        }

                        if self.is_eidting_radius_bottomleft {
                            let relative_raidus: f64 = self.style.border_bottom_left_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < 50_f64 && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_bottom_left_radius = format!("{}%", relative_raidus.trunc());
                            }
                        }

                        if self.is_eidting_radius_bottomright {
                            let relative_raidus: f64 = self.style.border_bottom_right_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < 50_f64 && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_bottom_right_radius = format!("{}%", relative_raidus.trunc());
                            }
                        }

                        
                    }
                    EditStates::Edit3D => {
                        if self.is_editing_3d {
                            if self.is_editing_3d_rotate_x {
                                let relative_x_rotation: f64 = self.style.transform.rotateX.try_to_f64().unwrap() - f64::from(offset_x) - f64::from(offset_y);
                                self.style.transform.rotateX = format!("{}deg", relative_x_rotation.trunc());
                            }
                            if self.is_editing_3d_rotate_y {
                                let relative_y_rotation: f64 = self.style.transform.rotateY.try_to_f64().unwrap() + f64::from(offset_x) + f64::from(offset_y);
                                self.style.transform.rotateY = format!("{}deg", relative_y_rotation.trunc());
                            }
                            if self.is_editing_3d_rotate_z {
                                let relative_z_rotation: f64 = self.style.transform.rotateZ.try_to_f64().unwrap() + f64::from(offset_x) + f64::from(offset_y);
                                self.style.transform.rotateZ = format!("{}deg", relative_z_rotation.trunc());
                            }
                        }
                    },
                    EditStates::Text => {
                        if self.is_editing_text_size {
                            let relative_text_size: f64 = self.style.font_size.try_to_f64().unwrap() + f64::from(offset_x) / 50_f64;
                            if relative_text_size > 0.5_f64 {
                                self.style.font_size = format!("{}rem", relative_text_size);
                            }
                        } else if self.is_editing_text_spacing_letters {
                            let relative_letter_spacing: f64 = self.style.letter_spacing.try_to_f64().unwrap() + f64::from(offset_x) / 50_f64;
                            self.style.letter_spacing = format!("{}em", relative_letter_spacing);

                        } else if self.is_editing_text_spacing_lines {
                            let relative_line_spacing: f64 = self.style.line_height.try_to_f64().unwrap() + f64::from(offset_x) / 50_f64;
                            self.style.line_height = format!("{}", relative_line_spacing);

                        } else if self.is_editing_text_spacing_words {
                            let relative_word_spacing: f64 = self.style.word_spacing.try_to_f64().unwrap() + f64::from(offset_x) / 50_f64;
                            self.style.word_spacing = format!("{}em", relative_word_spacing);

                        }
                    },
                    EditStates::None => {

                    }
                }

                // store previous mouse position
                self.previous_mouse_x = Some( parent_e.page_x() );
                self.previous_mouse_y = Some( parent_e.page_y() );

                true
            }
            Msg::AlignTextLeft => {
                self.style.text_align = "left".to_string();
                true
            },
            Msg::AlignTextCenter => {
                self.style.text_align = "center".to_string();
                true
            },
            Msg::AlignTextJustify => {
                self.style.text_align = "justify".to_string();
                true
            },
            Msg::AlignTextRight => {
                self.style.text_align = "right".to_string();
                true
            },
            Msg::StyleTextBold => {
                match self.style.font_weight.as_str() {
                    "normal" => {
                        self.style.font_weight = "bold".to_string();
                        true
                    },
                    &_ => {
                        self.style.font_weight = "normal".to_string();
                        true
                    }
                }
            },
            Msg::StyleTextItalic => {
                match self.style.font_style.as_str() {
                    "normal" => {
                        self.style.font_style = "italic".to_string();
                        true
                    },
                    &_ => {
                        self.style.font_style = "normal".to_string();
                        true
                    }
                }
            },
            Msg::StyleTextUnderline => {
                match self.style.text_decoration_line.as_str() {
                    "none" => {
                        self.style.text_decoration_line = "underline".to_string();
                        true
                    },
                    "underline" => {
                        self.style.text_decoration_line = "none".to_string();
                        true
                    }
                    &_ => {
                        info!("Invalid text-decoration-line value: {}", self.style.text_decoration_line);
                        false
                    }
                }
            },
            Msg::StyleTextSize => {false}, //onclick
            Msg::SpacingLetters => {false}, //onclick
            Msg::SpacingWords => {false}, //onclick
            Msg::SpacingLines => {false}, //onclick
            Msg::TextDirectionLRHorizontal => {
                self.style.writing_mode = "horizontal-tb".to_string();
                true
            },
            Msg::TextDirectionRLVertical => {
                self.style.writing_mode = "vertical-rl".to_string();
                true
            },
            Msg::TextDirectionLRVertical => {
                self.style.writing_mode = "vertical-lr".to_string();
                true
            }
            Msg::PickFont(select_event) => {
                let target_element = select_event.target().unwrap().dyn_into::<HtmlSelectElement>();
                self.style.font_family = target_element.unwrap().value();
                true
            },
            Msg::DisplayDropzones(e) => {
                let element_jdepth = self.event_jdepth(e);
                if element_jdepth.is_none() || ctx.props().jdepth != element_jdepth.unwrap() { return false }
                
                // Hide Dropzone on parent element
                if element_jdepth.unwrap() > 0 {
                    let direct_parent_link = ctx.link().get_parent().unwrap();
                    let direct_parent = direct_parent_link.try_downcast::<EditableElement>().unwrap();
                    direct_parent.send_message(Msg::HideDropzones);
                }

                self.is_dragging_in = true;
                true
            },
            Msg::HideDropzones => {
                // Hide Dropzone on parent element
                let direct_parent_link = ctx.link().get_parent().unwrap();
                let direct_parent = direct_parent_link.try_downcast::<EditableElement>();
                if let Some(editable_element) = direct_parent {
                    editable_element.send_message(Msg::HideDropzones)
                }

                self.is_dragging_in = false;
                true
            }
            Msg::EnableDropzones => {
                self.parent_link.send_message(PMsg::EnableDropzones);
                true
            },
            Msg::DisableDropzones => {
                self.parent_link.send_message(PMsg::DisableDropzones);
                true
            }
            Msg::DropInBottom => {
                let selected_link = self.parent_link.get_component().unwrap().selected_child.clone().unwrap();
                selected_link.send_message(Msg::Delete);

                let selected_component = selected_link.get_component().unwrap();
                self.children.push( selected_component.deref().clone() );

                info!("dropped in bottom");
                true
            },
            Msg::Noop => {false},
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Send this comoponents instrance context to the parent.
            let child_link = ctx.link().clone();
            self.parent_link.send_message( PMsg::ReceiveChildrenLink( child_link ));
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.render { return html!(<></>); }

        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        // Base styling
        let mut style = self.style.inline();

        // Conditional styling based on state
        let selected_style = SelectedStyle::create();
        if self.is_selected {
            style.push_str(&selected_style.inline());
        }
        // Tag based on given JType
        let mut tag = ctx.props().tag.clone();
        if tag.is_empty() {
            append_to_string!(
                match ctx.props().jtype {
                    JTypes::Div => { 
                        tag = "div"
                    },
                    JTypes::Text => {
                        tag = "p"
                    },
                    JTypes::Image => {
                        tag = "picture"
                    },
                }
            );
        }

        let mut dropzone_class = "".to_string();
        if !self.is_selected {
            dropzone_class = Judit_MakeDropzone::create().as_class(&self.document).unwrap();
        }

        html! {
            <@{tag} jrole = { "Judit_EditableElement" } class={ dropzone_class } jdepth={ctx.props().jdepth.to_string()} 
                ondrag = { link.callback( |e: DragEvent| { e.prevent_default(); Msg::Noop } )}
                ondragstart = { link.callback( |e: DragEvent| { e.prevent_default(); Msg::Noop } )}

                onmouseover = { link.callback( |e| Msg::DisplayDropzones(e) )}
                onmouseleave = { link.callback( |_| Msg::HideDropzones )}
                onclick = { link.callback( |e| Msg::Select(e) )}
                onmousedown = { link.batch_callback( |e| vec!(Msg::StartEditingWithCursor(e), Msg::EnableDropzones) )}
                onmouseup = { link.batch_callback( |e| vec!(Msg::StopEditingWithCursor(e), Msg::DisableDropzones) )}
                style={ style }>
                    // Nested Elements
                    { for ctx.props().children.iter() } // for any children that might be given 
                    { self.children.clone().into_iter().map(|c| // for layered EditableElements
                        html_nested!{<EditableElement jdepth={ ctx.props().jdepth + 1 } init_style={ c.style.clone() }></EditableElement>}).collect::<Html>() 
                    }
                    // Editing Stuff
                    if self.is_selected {
                        if self.style.width.try_to_f64().unwrap() > 20_f64 && self.style.height.try_to_f64().unwrap() > 20_f64 {
                            <EditableBorderRadiusSelector 
                                position = {ebrsPositions::TopLeft} 
                                border = { self.border_selector_style_topleft.clone() }/>
                            <EditableBorderRadiusSelector 
                                position = {ebrsPositions::TopRight}
                                border = { self.border_selector_style_topright.clone() }/>
                            <EditableBorderRadiusSelector 
                                position = {ebrsPositions::BottomLeft}
                                border = { self.border_selector_style_bottomleft.clone() }/>
                            <EditableBorderRadiusSelector 
                                position = {ebrsPositions::BottomRight}
                                border = { self.border_selector_style_bottomright.clone() }/>
                            if self. is_editing_3d {
                                <Transform3DSelector/>
                            }
                        }
                        
                        <DeleteButton parent_transform={ self.style.transform.clone() } onclick={ link.callback(|_| Msg::Delete )}/>
                        // Edit Controls below the EditableElements
                        <EditControls parent_transform={ self.style.transform.clone() }>
                            if self.is_editing_3d {
                                <Transform2DToggle onclick={link.callback(|_| Msg::Transform2DToggle )} />
                            } else {
                                <Transform3DToggle onclick={link.callback(|_| Msg::Transform3DToggle )} />
                            }
                        </EditControls>
                        <EditToolsPanel parent_transform={ self.style.transform.clone() } >
                            <TextEditPanel>
                                <AlignRightButton onclick={link.callback(|_| Msg::AlignTextRight )}/>
                                <AlignCenterButton onclick={link.callback(|_| Msg::AlignTextCenter )}/>
                                <AlignJustifyButton onclick={link.callback(|_| Msg::AlignTextJustify )}/>
                                <AlignLeftButton onclick={link.callback(|_| Msg::AlignTextLeft )}/>

                                <StyleBoldButton onclick={link.callback(|_| Msg::StyleTextBold )}/>
                                <StyleItalicButton onclick={link.callback(|_| Msg::StyleTextItalic )}/>
                                <StyleUnderlineButton onclick={link.callback(|_| Msg::StyleTextUnderline )}/>
                                <StyleSizeButton onclick={link.callback(|_| Msg::StyleTextSize )}/>

                                <SpacingLinesButton onclick={link.callback(|_| Msg::SpacingLines )}/>
                                <SpacingWordsButton onclick={link.callback(|_| Msg::SpacingWords )}/>
                                <SpacingLettersButton onclick={link.callback(|_| Msg::SpacingLetters )}/>
                                {   match self.style.writing_mode.as_str() {
                                        "horizontal-tb" => {
                                            html!{<RightAlignedVerticalText onclick={link.callback(|_| Msg::TextDirectionRLVertical )}/>}
                                        },
                                        "vertical-rl" => {
                                            html!{<LeftAlignedVerticalText onclick={link.callback(|_| Msg::TextDirectionLRVertical )}/>}
                                        },
                                        "vertical-lr" => {
                                            html!{<LeftAlignedHorizontalText onclick={link.callback(|_| Msg::TextDirectionLRHorizontal )}/>}
                                        },
                                        
                                        &_ => {
                                            todo!()
                                        }
                                    }
                                }
                                <FontPicker onchange={link.callback(|e| Msg::PickFont(e) )}/>
                            </TextEditPanel>
                        </EditToolsPanel>
                    }
                    if self.is_dragging_in && !self.is_selected && self.parent_link.get_component().unwrap().global_conds.is_dropzones_enabled {
                        // align edges
                        <DropzoneTop/>
                        <DropzoneLeft/>
                        <DropzoneRight/>
                        <DropzoneBottom onmouseup={link.callback(|_| Msg::DropInBottom )}/>
                        // align corners
                        <DropzoneTopLeft/>
                        <DropzoneTopRight/>
                        <DropzoneBottomLeft/>
                        <DropzoneBottomRight/>
                        // no align
                        <DropzoneNoAlign/>
                    }
            </@>
        }
    }
}