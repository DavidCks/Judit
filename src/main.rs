use yew::prelude::*;
use yew::html::{Scope};
use web_sys::{ MouseEvent, window };
use append_to_string::*;
use bevy_reflect::{ Reflect };
use rusty_css::*;

mod components;
pub use components::{ EditableElement::* };
pub use components::EditableElement::Msg as CMsg;
pub use components::EditableElement::JTypes as JTypes;
pub use components::Toolbar::Toolbar as Toolbar;

#[derive(Reflect)]
struct CanvasStyle {
    margin: String, 
    min_height: String,
    min_width: String,
    width: String,
    opacity: String,
    transform_style: String,
}

impl Style for CanvasStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                margin: "-8px", 
                min_height: "100vh",
                min_width: "100vw",
                width: "auto",
                opacity: "1",
                transform_style: "initial"
            }
        )
    }
}

pub enum Msg {
    ReceiveSelectedChildLink(Scope<EditableElement>),
    ReceiveChildrenLink(Scope<EditableElement>),
    PropagateCursorMove(MouseEvent),
    StopAllEditing(MouseEvent),
    AddElement(JTypes),
    EnableDropzones,
    DisableDropzones,
    IncrementGlobalCouter,
    DecrementGlobalCounter,
}

struct App {
    style: CanvasStyle,
    children_links: Vec<Scope<EditableElement>>,
    selected_child: Option<Scope<EditableElement>>,
    previous_selected_child: Option<Scope<EditableElement>>,
    children: Vec<Html>,
    global_conds: GlobalConditions,
    global_counter: u32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GlobalConditions {
    is_dropzones_enabled: bool,
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            style: CanvasStyle::create(),
            children_links: Vec::new(),
            selected_child: None,
            previous_selected_child: None,
            children: vec!( html!(<EditableElement></EditableElement>) ),
            global_conds: GlobalConditions {
                is_dropzones_enabled: false,
            },
            global_counter: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveSelectedChildLink(new_scope) => {
                self.previous_selected_child = self.selected_child.clone();

                // Deselect current child
                if let Some(current_scope) = self.selected_child.as_ref() {
                    let selected_jdepth = current_scope.get_component().unwrap().jdepth;
                    let new_selected_jdepth = new_scope.get_component().unwrap().jdepth;
                    if new_selected_jdepth > selected_jdepth {
                        current_scope.send_message_batch(vec![CMsg::Deselect, CMsg::HideOverlay]);
                    } else  {
                        current_scope.send_message_batch(vec![CMsg::Deselect]);
                    }
                }
                self.selected_child = Some( new_scope.clone() );
                new_scope.send_message(CMsg::ShowOverlayUnchecked);
                false
            }
            Msg::ReceiveChildrenLink(child_scope) => {
                self.children_links.push(child_scope);
                false
            }
            Msg::PropagateCursorMove(e) => {

                if let Some(link) = self.selected_child.as_ref() {
                    link.send_message(CMsg::ReceiveCursorMove(e));
                }
                false
            }
            Msg::StopAllEditing(e) => {

                if let Some(link) = self.selected_child.as_ref() {
                    link.send_message(CMsg::StopEditingWithCursor(e));
                }
                false
            }
            Msg::AddElement(jtype) => {
                self.children.insert(0, html!(<EditableElement jtype={jtype} />));
                ctx.link().send_message(Msg::IncrementGlobalCouter);
                true
            }
            Msg::EnableDropzones => {
                self.global_conds.is_dropzones_enabled = true;
                true
            },
            Msg::DisableDropzones => {
                self.global_conds.is_dropzones_enabled = false;
                true
            }
            Msg::IncrementGlobalCouter => { self.global_counter += 1; false },
            Msg::DecrementGlobalCounter => { self.global_counter -= 1; false },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        let window = window().expect("No global `window` found");
        let document = window.document().expect("couldn't get `document");

        let class_name = self.style.as_class(&document);

        html! {
            <>
                <main
                    onmousemove = { link.callback( |e| Msg::PropagateCursorMove(e) )}
                    onmouseup = { link.callback( |e| Msg::StopAllEditing(e) )}
                    class = { class_name.unwrap_or_default() }>
                    { for self.children.clone().into_iter()  }
                    //add component that grabs the style tag, adds content-editable + display: block so it can be live edited
                </main>
                <Toolbar />
            </>
        }
    }
}

fn main() {
    //* Debuggig
    wasm_logger::init(wasm_logger::Config::default());

    //launch component
    yew::Renderer::<App>::new().render();
} 
