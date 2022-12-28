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

    opacity: String,
    z_index: String,
}

impl Style for ComponentStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                position: "absolute",
                box_sizing: "border-box",
                top: "20px",
                left: "50px",
                width: "200px",
                height: "200px",
                background_color: "#EEEEEE",
                transform_origin: "50% 50%",
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
                opacity: "1",
                z_index: "0",
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
    PutInBack,
    ResetPutInBack,

    // - with alignment
    DropInTopLeft,
    DropInTopRight,
    DropInBottomLeft,
    DropInBottomRight,
    DropInTop,
    DropInLeft,
    DropInRight,
    DropInBottom,
    // - without alignment
    DropInNoAlign(MouseEvent),
    

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

    #[prop_or_default]
    pub jelcount: u32,
}

#[derive(Clone)]
pub struct EditableElement {

    parent_link: Scope<super::super::App>,
    document: Document,
    children: Vec<Scope<EditableElement>>,
    jtype: JTypes,
    pub jdepth: u32,
    tag: String,
    inner_html: Html,

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

    edit_mode_3d_rx_ry: bool, // (/\ | \/ = rotateX, < | > = rotateY) determines if mouse movements should be mapped for x and y simultaneously, disregarding the clicked disk 
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

    // init style to be passed to child upon its creation
    child_init_style: ComponentStyle,
    _immediate_parent_link: Option<Scope<EditableElement>>,

    // state
    previous_mouse_x: Option<i32>,
    previous_mouse_y: Option<i32>,
    offset_x: i32,
    offset_y: i32,
    offset_cursor_pos: (i32, i32),
    z_index_buffer: String,
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

    fn get_selected_link(&self) -> Option<Scope<EditableElement>>{
        self.parent_link.get_component().unwrap().selected_child.clone()
    }

    fn hide_and_clone_selected(&self) -> EditableElement {
        let selected_link = self.get_selected_link().unwrap();
        selected_link.send_message(Msg::Delete);
        let ref_selected_component = selected_link.get_component().unwrap();
        ref_selected_component.deref().clone()
    }

    fn from_link(link: &Scope<EditableElement>, jdepth: u32) -> yew::virtual_dom::VNode {
        let mut return_html = html!();
        if let Some(ee) = link.get_component() {
            return_html = html!{
                <EditableElement 
                jdepth={ jdepth }
                jtype={ ee.jtype.clone() }
                tag={ ee.tag.clone() }>
                    { ee.inner_html.clone() }
                </EditableElement>
            };
        }
        return_html
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

        let mut init_style = ctx.props().init_style.clone();
        let mut _immediate_parent_link = None;
        if ctx.props().jdepth > 0 {
            _immediate_parent_link = ctx.link().get_parent().expect("No Parent found").clone()
                .try_downcast::<EditableElement>();
            let uwraped__immediate_parent_link = _immediate_parent_link.clone().unwrap();
            let immediate_parent_comp = uwraped__immediate_parent_link.get_component().unwrap();
            init_style = immediate_parent_comp.child_init_style.clone();
        }
        init_style.z_index = ctx.props().jelcount.to_string();

        let initial_border_selector_style = append_to_string!( 
            BorderSelectorStyle { 
                border_width: "3px", 
                border_color: "#3f3f3f", 
                border_radius: "100%" 
            }
        );

        // Tag and initial content based on given JType
        let mut tag = ctx.props().tag.clone();
        let mut inner_html = html!();
        if tag.is_empty() {
            append_to_string!(
                match ctx.props().jtype {
                    JTypes::Div => { 
                        tag = "div";
                        inner_html = html!();
                    },
                    JTypes::Text => {
                        info!("fuck you");
                        tag = "p";
                        inner_html = html!({ "Text Node" });
                    },
                    JTypes::Image => {
                        tag = "picture";
                        inner_html = html!(<img alt={"Image Node"} src={""}/>);
                    },
                }
            );
        }

        Self {
            parent_link: parent_link.unwrap(),
            document: window().unwrap().document().unwrap(),
            jtype: ctx.props().jtype.clone(),
            jdepth: ctx.props().jdepth.clone(),
            tag: tag,
            inner_html: inner_html,
            children: Vec::new(),

            style: init_style,
            border_selector_style_topleft: initial_border_selector_style.clone(),
            border_selector_style_topright: initial_border_selector_style.clone(),
            border_selector_style_bottomleft: initial_border_selector_style.clone(),
            border_selector_style_bottomright: initial_border_selector_style.clone(),

            previous_mouse_x: None,
            previous_mouse_y: None,
            offset_x: 0,
            offset_y: 0,
            offset_cursor_pos: (0, 0),

            editing_state: EditStates::None,

            is_movable: false,

            is_resizeable: false,
            is_resizeable_left: false,
            is_resizeable_right: false,
            is_resizeable_top: false,
            is_resizeable_bottom: false,

            edit_mode_3d_rx_ry: true,
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
            z_index_buffer: "0".to_string(),

            child_init_style: ctx.props().init_style.clone(),
            _immediate_parent_link: _immediate_parent_link,
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

                self.offset_cursor_pos = (e.offset_x(), e.offset_y());

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
                            ctx.link().send_message(Msg::PutInBack);
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

                ctx.link().send_message(Msg::ResetPutInBack);

                false
            }
            Msg::ReceiveCursorMove(parent_e) => {

                // calculate offset from the previous mouse event
                let offset_x = parent_e.page_x() - self.previous_mouse_x.unwrap_or( parent_e.page_x() );
                let offset_y = parent_e.page_y() - self.previous_mouse_y.unwrap_or( parent_e.page_y() );

                self.offset_x = offset_x;
                self.offset_y = offset_y;

                if !self.is_selected {
                    return false;
                }

                (|| match self.editing_state {
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

                        let max_radius = self.style.width.try_to_f64().unwrap();

                        if self.is_eidting_radius_topleft {
                            let relative_raidus: f64 = self.style.border_top_left_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < max_radius && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_top_left_radius = format!("{}px", relative_raidus.trunc());
                            }
                        }

                        if self.is_eidting_radius_topright {
                            let relative_raidus: f64 = self.style.border_top_right_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < max_radius && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_top_right_radius = format!("{}px", relative_raidus.trunc());
                            }
                        }

                        if self.is_eidting_radius_bottomleft {
                            let relative_raidus: f64 = self.style.border_bottom_left_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < max_radius && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_bottom_left_radius = format!("{}px", relative_raidus.trunc());
                            }
                        }

                        if self.is_eidting_radius_bottomright {
                            let relative_raidus: f64 = self.style.border_bottom_right_radius.try_to_f64().unwrap() + f64::from(offset_x);

                            if relative_raidus.trunc() < max_radius && relative_raidus.trunc() >= 0_f64 {
                                self.style.border_bottom_right_radius = format!("{}px", relative_raidus.trunc());
                            }
                        }
                    }
                    EditStates::Edit3D => {
                        if self.is_editing_3d {
                            if self.edit_mode_3d_rx_ry {
                                let relative_x_rotation: f64 = self.style.transform.rotateX.try_to_f64().unwrap() - f64::from(offset_y);
                                self.style.transform.rotateX = format!("{}deg", relative_x_rotation.trunc());

                                let relative_y_rotation: f64 = self.style.transform.rotateY.try_to_f64().unwrap() + f64::from(offset_x);
                                self.style.transform.rotateY = format!("{}deg", relative_y_rotation.trunc());

                                return;
                            } 

                            // standard 3d edit mode
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
                })();

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

                if !self.is_selected && self.parent_link.get_component().unwrap().global_conds.is_dropzones_enabled {
                    info!("{}", self.parent_link.get_component().unwrap().global_conds.is_dropzones_enabled);
                    self.is_dragging_in = true;
                    self.style.opacity = ".5".to_string();
                }
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
                self.style.opacity = "1".to_string();
                true
            }
            Msg::EnableDropzones => {
                self.parent_link.send_message(PMsg::EnableDropzones);
                true
            },
            Msg::DisableDropzones => {
                ctx.link().send_message(Msg::HideDropzones);
                self.parent_link.send_message(PMsg::DisableDropzones);
                true
            }
            Msg::DropInTopLeft => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_width = self.style.width.try_to_f64().unwrap() / 2_f64;
                let parent_half_height = self.style.height.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", -parent_border_width.trunc());
                selected_component.style.left = format!("{}px", -parent_border_width.trunc());
                selected_component.style.width = format!("{}px", parent_half_width.trunc());
                selected_component.style.height = format!("{}px", parent_half_height.trunc());
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                true
            },
            Msg::DropInTopRight => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_width = self.style.width.try_to_f64().unwrap() / 2_f64;
                let parent_half_height = self.style.height.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", -parent_border_width.trunc());
                selected_component.style.left = format!("{}px", (parent_half_width - parent_border_width).trunc());
                selected_component.style.width = format!("{}px", parent_half_width.trunc());
                selected_component.style.height = format!("{}px", parent_half_height.trunc());
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::DropInBottomLeft => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_width = self.style.width.try_to_f64().unwrap() / 2_f64;
                let parent_half_height = self.style.height.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", (parent_half_height - parent_border_width).trunc());
                selected_component.style.left = format!("{}px", -parent_border_width.trunc());
                selected_component.style.width = format!("{}px", parent_half_width.trunc());
                selected_component.style.height = format!("{}px", parent_half_height.trunc());
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::DropInBottomRight => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_width = self.style.width.try_to_f64().unwrap() / 2_f64;
                let parent_half_height = self.style.height.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", (parent_half_height - parent_border_width).trunc());
                selected_component.style.left = format!("{}px", (parent_half_width - parent_border_width).trunc());
                selected_component.style.width = format!("{}px", parent_half_width.trunc());
                selected_component.style.height = format!("{}px", parent_half_height.trunc());
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::DropInTop => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_height = self.style.height.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", -parent_border_width.trunc());
                selected_component.style.left = format!("{}px", -parent_border_width.trunc());
                selected_component.style.width = self.style.width.clone();
                selected_component.style.height = format!("{}px", parent_half_height.trunc());
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::DropInLeft => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_width = self.style.width.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", -parent_border_width.trunc());
                selected_component.style.left = format!("{}px", -parent_border_width.trunc());
                selected_component.style.width = format!("{}px", parent_half_width.trunc());
                selected_component.style.height = self.style.height.clone();
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::DropInRight => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_width = self.style.width.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", -parent_border_width.trunc());
                selected_component.style.left = format!("{}px", (parent_half_width - parent_border_width).trunc());
                selected_component.style.width = format!("{}px", parent_half_width.trunc());
                selected_component.style.height = self.style.height.clone();
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::DropInBottom => {
                let mut selected_component = self.hide_and_clone_selected();

                let parent_half_height = self.style.height.try_to_f64().unwrap() / 2_f64;
                let parent_border_width = self.style.border_width.try_to_f64().unwrap();

                selected_component.style.top = format!("{}px", (parent_half_height - parent_border_width).trunc());
                selected_component.style.left = format!("{}px", -parent_border_width.trunc());
                selected_component.style.width = self.style.width.clone();
                selected_component.style.height = format!("{}px", parent_half_height.trunc());
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::DropInNoAlign(e) => {
                let mut selected_component = self.hide_and_clone_selected();                

                let target = e.target_dyn_into::<Element>().unwrap();
                let parent = target.parent_element().unwrap();
                let bounds = parent.get_bounding_client_rect();
                let x = e.client_x() - bounds.left() as i32 - selected_component.offset_cursor_pos.0;
                let y = e.client_y() - bounds.top() as i32 - selected_component.offset_cursor_pos.1;

                selected_component.style.top = format!("{}px", y);
                selected_component.style.left = format!("{}px", x);
                self.child_init_style = selected_component.style.clone();

                // let render_component = selected_component.copy_as_html(self.jdepth + 1);
                let parent_comp = self.parent_link.get_component().unwrap();
                if let Some(selected_child) = parent_comp.selected_child.clone() {
                    self.children.insert(0, selected_child );
                    self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                }

                self.parent_link.send_message(PMsg::IncrementGlobalCouter);
                info!("dropped topright");
                true
            },
            Msg::PutInBack => {
                self.z_index_buffer = self.style.z_index.clone();
                self.style.z_index = "0".to_string();
                true
            },
            Msg::ResetPutInBack => {
                self.style.z_index = self.z_index_buffer.clone();
                self.z_index_buffer = "1".to_string();
                true
            },
            Msg::Noop => {false},
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
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

        html! {
            <@{tag} jrole = { "Judit_EditableElement" } 
                    jdepth={ctx.props().jdepth.to_string()} 
                ondragstart = { link.callback( |e: DragEvent| { e.prevent_default(); Msg::Noop } )}
                ondragend = { link.callback( |e: DragEvent| { e.prevent_default(); Msg::Noop } )}

                onmouseover = { link.callback( |e| Msg::DisplayDropzones(e) )}
                onmouseleave = { link.callback( |_| Msg::HideDropzones )}
                onclick = { link.callback( |e| { Msg::Select(e) } )}
                onmousedown = { link.batch_callback( |e| {info!("onclick"); vec!(Msg::StartEditingWithCursor(e), Msg::EnableDropzones)} )}
                onmouseup = { link.batch_callback( |e| vec!(Msg::StopEditingWithCursor(e), Msg::DisableDropzones) )}
                style={ style }>
                    // Nested Elements
                    { self.inner_html.clone() } //initial inner html based on tag / jtype
                    { for ctx.props().children.iter() } // for any children that might be given 
                    { self.children.iter().map(|child| EditableElement::from_link(child, self.jdepth + 1)).collect::<Html>()} // for nested stuff
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
                    if self.is_dragging_in {
                        // align edges
                        <DropzoneTop onmouseup={link.callback(|_| Msg::DropInTop )}/>
                        <DropzoneLeft onmouseup={link.callback(|_| Msg::DropInLeft )}/>
                        <DropzoneRight onmouseup={link.callback(|_| Msg::DropInRight )}/>
                        <DropzoneBottom onmouseup={link.callback(|_| Msg::DropInBottom )}/>
                        // align corners
                        <DropzoneTopLeft onmouseup={link.callback(|_| Msg::DropInTopLeft )}/>
                        <DropzoneTopRight onmouseup={link.callback(|_| Msg::DropInTopRight )}/>
                        <DropzoneBottomLeft onmouseup={link.callback(|_| Msg::DropInBottomLeft )}/>
                        <DropzoneBottomRight onmouseup={link.callback(|_| Msg::DropInBottomRight )}/>
                        // no align
                        <DropzoneNoAlign onmouseup={link.callback(|e| Msg::DropInNoAlign(e) )}/>
                    }
            </@>
        }
    }
}