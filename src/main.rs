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
            children: vec!( html!(<EditableElement jelcount={1}></EditableElement>) ),
            global_conds: GlobalConditions {
                is_dropzones_enabled: false,
            },
            global_counter: 2,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                self.children.insert(0, html!(<EditableElement jelcount={ self.global_counter } jtype={jtype} />));
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
                    onmouseup = { link.batch_callback( |e| vec![Msg::StopAllEditing(e), Msg::DisableDropzones ])}
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
