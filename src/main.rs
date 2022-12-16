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
}

struct App {
    style: CanvasStyle,
    children_links: Vec<Scope<EditableElement>>,
    selected_child: Option<Scope<EditableElement>>,
    children: Vec<Html>,
    is_dropzones_enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct GlobalConditions {
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
            children: vec!( html!(<EditableElement />) ),
            is_dropzones_enabled: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveSelectedChildLink(child_scope) => {
                // Deselect current child
                if let Some(link) = self.selected_child.as_ref() {
                    link.send_message(CMsg::Deselect);
                }
                self.selected_child = Some( child_scope );
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
                self.children.push( html!(<EditableElement jtype={jtype}/>) ); 
                true
            }
            Msg::EnableDropzones => {
                self.is_dropzones_enabled = true;
                false
            },
            Msg::DisableDropzones => {
                self.is_dropzones_enabled = false;
                false
            }
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
    yew::start_app::<App>();
} 
