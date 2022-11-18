//use log::info;
use yew::{prelude::*};
use yew::html::Scope;
use bevy_reflect::{ Reflect };
use append_to_string::*;
use web_sys::{ DragEvent, MouseEvent };
use rusty_css::*;
use super::super::Msg as PMsg;

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
                transform_origin: "bottom right 20px",
                top: "0px",
                left: "0px",
                width: "100px",
                height: "100px",
                background_color: "lightgray",
                transform: Transform { 
                    skewX: "10deg",
                    skewY: "0deg",
                    translateX: "0px",
                },
                border_radius: "2px",
            }
        )
    }
}

// Component Fuctions
pub enum Msg {

    // Receive Parents mouse move event
    ReceiveCursorMove(MouseEvent),

    // Drag and Drop
    StartEditingWithCursor(DragEvent),
    StopEditingWithCursor(MouseEvent),

    Select,
    Deselect,
}

pub struct EditableElement {

    parent_link: Scope<super::super::App>,

    style: ComponentStyle,

    // contols
    is_movable: bool, 

    is_resizeable: bool,
    is_resizeable_left: bool,
    is_resizeable_right: bool,
    is_resizeable_top: bool,
    is_resizeable_bottom: bool,

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

        Self {
            parent_link: parent_link.downcast::<super::super::App>(),

            style: ComponentStyle::create(),

            previous_mouse_x: None,
            previous_mouse_y: None,

            is_movable: false,

            is_resizeable: false,
            is_resizeable_left: false,
            is_resizeable_right: false,
            is_resizeable_top: false,
            is_resizeable_bottom: false,

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

                // reset previous mouse position
                self.previous_mouse_x = None;
                self.previous_mouse_y = None;
                true
            }
            Msg::StartEditingWithCursor(e) => {
                e.prevent_default();

                if !self.is_selected {
                    ctx.link().send_message(Msg::Select);
                }

                // determine wether the border has been clicked or the box
                let resize_range = 5;

                // horizontal
                if e.offset_x() < resize_range { // left side
                    self.is_resizeable = true;
                    self.is_resizeable_left = true;
                } else if self.style.width.try_to_f64().unwrap() - <i32 as Into<f64>>::into(resize_range) < e.offset_x().into() { // right side
                    self.is_resizeable = true;
                    self.is_resizeable_right = true;
                }

                // vertical
                if e.offset_y() < resize_range { // top side
                    self.is_resizeable = true;
                    self.is_resizeable_top = true;
                } else if self.style.height.try_to_f64().unwrap() - <i32 as Into<f64>>::into(resize_range) < e.offset_y().into() { // bottom side
                    self.is_resizeable = true;
                    self.is_resizeable_bottom = true;
                }

                if !self.is_resizeable {
                    self.is_movable =  true;
                }

                false
            }
            Msg::StopEditingWithCursor(e) => {
                e.prevent_default();
                self.is_movable = false;

                self.is_resizeable = false;
                self.is_resizeable_bottom = false;
                self.is_resizeable_left = false;
                self.is_resizeable_right = false;
                self.is_resizeable_top = false;

                false
            }
            Msg::ReceiveCursorMove(parent_e) => {

                if !self.is_selected {
                    return false;
                }

                // calculate offset from the previous mouse event
                let offset_x = parent_e.page_x() - self.previous_mouse_x.unwrap_or( parent_e.page_x() );
                let offset_y = parent_e.page_y() - self.previous_mouse_y.unwrap_or( parent_e.page_y() );

                if self.is_movable {

                    // calculate relative x and y positions for the offset in css
                    let relative_x: f64 = self.style.left.try_to_f64().unwrap() + f64::from(offset_x);
                    let relative_y: f64 = self.style.top.try_to_f64().unwrap() + f64::from(offset_y);

                    // assign new position and assign it to the style
                    self.style.left = format!("{}px", relative_x.trunc());
                    self.style.top = format!("{}px", relative_y.trunc());

                } else if self.is_resizeable {

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
        
        log::info!("top: {}\nmoveable: {}\nresizeable: {}\nrLeft: {}; rRight: {}; rTop{}; rBot{}; \nselected: {}\nprev_x: {}; prev_y: {};", self.style.top, self.is_movable, self.is_resizeable, self.is_resizeable_left, self.is_resizeable_right, self.is_resizeable_top, self.is_resizeable_bottom, self.is_selected, self.previous_mouse_x.unwrap_or_default(), self.previous_mouse_y.unwrap_or_default());

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
                    <p style={"text-align: center"}>{ &self.style.top }</p>
                    <p style={"text-align: center"}>{ &self.style.left }</p>
            </div>
        }
    }
}