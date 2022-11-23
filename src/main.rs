use yew::prelude::*;
use yew::html::Scope;
use web_sys::{ MouseEvent, window };
use append_to_string::*;
use bevy_reflect::{ Reflect };
use rusty_css::*;

mod components;
pub use components::{ EditableElement::* };
pub use components::EditableElement::Msg as CMsg;

pub enum Msg {
    ReceiveSelectedChildLink(Scope<EditableElement>),
    ReceiveChildrenLink(Scope<EditableElement>),
    PropagateCursorMove(MouseEvent),
    StopAllEditing(MouseEvent),
}

#[derive(Reflect)]
struct CanvasStyle {
    margin: String, 
    height: String,
    width: String,
    background_color: String,
    opacity: String,
    background_image: String,
    background_size: String,
}

impl Style for CanvasStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                margin: "-8px", 
                height: "100vh",
                width: "100vw",
                background_color: "#f9f9f9",
                opacity: "1",
                background_image: "radial-gradient(#b1b1b1 1px, #f9f9f9 1px)",
                background_size: "20px 20px",
            }
        )
    }
}
struct App {
    style: CanvasStyle,
    children: Vec<Scope<EditableElement>>,
    selected_child: Option<Scope<EditableElement>>,
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            style: CanvasStyle::create(),
            children: Vec::new(),
            selected_child: None,
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
                self.children.push(child_scope);
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        let window = window().expect("No global `window` found");
        let document = window.document().expect("couldn't get `document");

        let class_name = self.style.as_class(&document);

        html! {
            <main
                onmousemove = { link.callback( |e| Msg::PropagateCursorMove(e) )}
                onmouseup = { link.callback( |e| Msg::StopAllEditing(e) )}
                class = { class_name.unwrap_or_default() }>
                <EditableElement />
                <EditableElement />
                <EditableElement />
                //add component that grabs the style tag, adds content-editable + display: block so it can be live edited
            </main>
        }
    }
}

fn main() {
    //* Debuggig
    wasm_logger::init(wasm_logger::Config::default());

    //launch component
    yew::start_app::<App>();
} 
