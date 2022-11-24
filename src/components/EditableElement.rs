use log::info;
//use log::info;
use yew::{prelude::*};
use yew::html::Scope;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ DragEvent, MouseEvent };
use rusty_css::*;
use super::super::Msg as PMsg;

use super::sub_components::EditableBorderRadiusSelecor::EditableBorderRadiusSelector as EditableBorderRadiusSelector;
use super::sub_components::EditableBorderRadiusSelecor::Positions as ebrsPositions;
use super::sub_components::EditableBorderRadiusSelecor::BorderSelectorStyle;

// external styles
use super::static_styles::Selected::Selected as SelectedStyle;

#[allow(non_snake_case)]
#[derive(Reflect)]
struct Transform {
    skewX: String,
    skewY: String,
    translateX: String,
}

#[allow(non_snake_case)]
#[derive(Reflect)]
struct ComponentStyle {
    position: String,
    box_sizing: String,

    top: String,
    left: String,
    width: String,
    height: String,

    background_color: String,

    transform_origin: String,
    transform: Transform,

    border_radius: String,
}

impl Style for ComponentStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                position: "absolute",
                box_sizing: "border-box",
                transform_origin: "bottom right 20px",
                top: "0px",
                left: "0px",
                width: "100px",
                height: "100px",
                background_color: "lightgray",
                transform: Transform { 
                    skewX: "0deg",
                    skewY: "0deg",
                    translateX: "0px",
                },
                border_radius: "10px",
            }
        )
    }
}

pub enum EditStates {
    None,
    Move,
    Resize,
    BorderRadius,
}

// Component Fuctions
pub enum Msg {
    // Drag and Drop / resize
    StartEditingWithCursor(DragEvent),
    StopEditingWithCursor(MouseEvent),
    
    // Receive Parents mouse move event
    ReceiveCursorMove(MouseEvent),

    // border radius
    StartEditingBorderRadius(ebrsPositions, MouseEvent),
    StopEditingBorderRadius(ebrsPositions, MouseEvent),

    Select,
    Deselect,
}

pub struct EditableElement {

    parent_link: Scope<super::super::App>,

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
    is_editing_radius_linked: bool,

    is_selected: bool,

    // state
    previous_mouse_x: Option<i32>,
    previous_mouse_y: Option<i32>,
}

impl Component for EditableElement {
    type Properties = ();
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {

        let parent_link = ctx.link().get_parent().expect("No Parent found").clone();

        let initial_border_selector_style = append_to_string!( 
            BorderSelectorStyle { 
                border_width: "3px", 
                border_color: "darkgray", 
                border_radius: "100%" 
            }
        );

        Self {
            parent_link: parent_link.downcast::<super::super::App>(),

            style: ComponentStyle::create(),
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
            is_editing_radius_linked: true,

            is_selected: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select => {
                
                if !self.is_selected {
                    // Send this comoponents instrance context to the parent.
                    let child_link = ctx.link().clone();
                    self.parent_link.send_message( PMsg::ReceiveSelectedChildLink( child_link ));
                    
                    self.is_selected = true;
                    return true;
                }
                false
            }
            Msg::Deselect => {
                self.is_selected = false;
                self.editing_state = EditStates::None;

                // reset previous mouse position
                self.previous_mouse_x = None;
                self.previous_mouse_y = None;
                true
            }
            Msg::StartEditingWithCursor(e) => {
                e.prevent_default();

                ///////////////////////////////////////////////////////////////////////////
                // edit elements width / height or move element based on cursor position //
                ///////////////////////////////////////////////////////////////////////////

                // select if not already selected
                if !self.is_selected {
                    ctx.link().send_message(Msg::Select);
                }



                // determine wether the border has been clicked or the box
                let resize_range = 5;

                // horizontal resize
                if e.offset_x() < resize_range { // left side
                    self.is_resizeable = true;
                    self.editing_state = EditStates::Resize;
                    self.is_resizeable_left = true;
                } else if self.style.width.try_to_f64().unwrap() - <i32 as Into<f64>>::into(resize_range) < e.offset_x().into() { // right side
                    self.is_resizeable = true;
                    self.editing_state = EditStates::Resize;
                    self.is_resizeable_right = true;
                }

                // vertical resize
                if e.offset_y() < resize_range { // top side
                    self.is_resizeable = true;
                    self.editing_state = EditStates::Resize;
                    self.is_resizeable_top = true;
                } else if self.style.height.try_to_f64().unwrap() - <i32 as Into<f64>>::into(resize_range) < e.offset_y().into() { // bottom side
                    self.is_resizeable = true;
                    self.editing_state = EditStates::Resize;
                    self.is_resizeable_bottom = true;
                }

                // free movement
                if !self.is_resizeable {
                    self.is_movable =  true;
                    self.editing_state = EditStates::Move;
                }

                false
            }
            Msg::StopEditingWithCursor(e) => {
                e.prevent_default();
                self.is_movable = false;
                self.editing_state = EditStates::None;

                self.is_resizeable = false;
                self.is_resizeable_bottom = false;
                self.is_resizeable_left = false;
                self.is_resizeable_right = false;
                self.is_resizeable_top = false;
                
                false
            }
            Msg::StartEditingBorderRadius(pos, e) => {
                
                self.is_eidting_radius = true;
                self.editing_state = EditStates::BorderRadius;
                match pos {
                    ebrsPositions::TopLeft => {
                        self.is_eidting_radius_topleft = true;
                    }
                    ebrsPositions::TopRight => {
                        self.is_eidting_radius_topright = true;
                    }
                    ebrsPositions::BottomLeft => {
                        self.is_eidting_radius_bottomleft= true;
                    }
                    ebrsPositions::BottomRight => {
                        self.is_eidting_radius_bottomright= true;
                    }
                }
                false
            }
            Msg::StopEditingBorderRadius(pos, _e) => {
                
                self.is_eidting_radius = false;
                self.editing_state = EditStates::None;
                match pos {
                    ebrsPositions::TopLeft => {
                        self.is_eidting_radius_topleft = false;
                    }
                    ebrsPositions::TopRight => {
                        self.is_eidting_radius_topright = false;
                    }
                    ebrsPositions::BottomLeft => {
                        self.is_eidting_radius_bottomleft = false;
                    }
                    ebrsPositions::BottomRight => {
                        self.is_eidting_radius_bottomright = false;
                    }
                }
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
                        let relative_raidus: f64 = self.style.border_radius.try_to_f64().unwrap() + f64::from(offset_x);

                        self.style.border_radius = format!("{}px", relative_raidus.trunc());
                    }
                    EditStates::None => {

                    }
                }

                // store previous mouse position
                self.previous_mouse_x = Some( parent_e.page_x() );
                self.previous_mouse_y = Some( parent_e.page_y() );

                true
            }
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
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        
        //log::info!("top: {}\nmoveable: {}\nresizeable: {}\nrLeft: {}; rRight: {}; rTop{}; rBot{}; \nselected: {}\nprev_x: {}; prev_y: {};", self.style.top, self.is_movable, self.is_resizeable, self.is_resizeable_left, self.is_resizeable_right, self.is_resizeable_top, self.is_resizeable_bottom, self.is_selected, self.previous_mouse_x.unwrap_or_default(), self.previous_mouse_y.unwrap_or_default());

        // Base styling
        let mut style = format!("{}",  self.style.inline());

        // Conditional styling based on state
        let selected_style = SelectedStyle::create();
        if self.is_selected {
            style.push_str(&selected_style.inline());
        }

        html! {
            <div draggable="true"
                onclick = { link.callback( |_| Msg::Select )}
                ondragstart = { link.callback( |e| Msg::StartEditingWithCursor(e) )}
                onmouseup = { link.callback( |e| Msg::StopEditingWithCursor(e) )}
                style={ style }>
                    if self.is_selected && 
                       self.style.width.try_to_f64().unwrap() > 20_f64 && 
                       self.style.height.try_to_f64().unwrap() > 20_f64 {
                        <EditableBorderRadiusSelector 
                            position = {ebrsPositions::TopLeft} 
                            border = { self.border_selector_style_topleft.clone() }
                            onmousedown = { link.callback( |e| { Msg::StartEditingBorderRadius(ebrsPositions::TopLeft, e) } )}
                            onmouseup = { link.callback( |e| { Msg::StopEditingBorderRadius(ebrsPositions::TopLeft, e) } )}/>
                        <EditableBorderRadiusSelector 
                            position = {ebrsPositions::TopRight}
                            border = { self.border_selector_style_topright.clone() }
                            onmousedown = { link.callback( |e| Msg::StartEditingBorderRadius(ebrsPositions::TopRight, e) )}
                            onmouseup = { link.callback( |e| { Msg::StopEditingBorderRadius(ebrsPositions::TopRight, e) } )}/>
                        <EditableBorderRadiusSelector 
                            position = {ebrsPositions::BottomLeft}
                            border = { self.border_selector_style_bottomleft.clone() }
                            onmousedown = { link.callback( |e| Msg::StartEditingBorderRadius(ebrsPositions::BottomLeft, e) )}
                            onmouseup = { link.callback( |e| { Msg::StopEditingBorderRadius(ebrsPositions::BottomLeft, e) } )}/>
                        <EditableBorderRadiusSelector 
                            position = {ebrsPositions::BottomRight}
                            border = { self.border_selector_style_bottomright.clone() }
                            onmousedown = { link.callback( |e| Msg::StartEditingBorderRadius(ebrsPositions::BottomRight, e) )}
                            onmouseup = { link.callback( |e| { Msg::StopEditingBorderRadius(ebrsPositions::BottomRight, e) } )}/>
                    }
            </div>
        }
    }
}