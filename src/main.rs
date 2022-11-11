use yew::prelude::*;
use yew::html::Scope;
use web_sys::{ MouseEvent };

mod components;
pub use components::{ EditableElement::* };
pub use components::EditableElement::Msg as CMsg;

pub enum Msg {
    ReceiveSelectedChildLink(Scope<EditableElement>),
    ReceiveChildrenLink(Scope<EditableElement>),
    PropagateCursorMove(MouseEvent),
    StopAllEditing(MouseEvent),
}
struct App {
    children: Vec<Scope<EditableElement>>,
    selected_child: Option<Scope<EditableElement>>,
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
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
        html! {
            <div
                onmousemove = { link.callback( |e| Msg::PropagateCursorMove(e) )}
                onmouseup = { link.callback( |e| Msg::StopAllEditing(e) )}
                style={"margin: -8px; height: 100vh; width: 100vw; background-color: #f9f9f9;
                opacity: 1;
                background-image: radial-gradient(#b1b1b1 1px, #f9f9f9 1px);
                background-size: 20px 20px;"}>
                <EditableElement />
                <EditableElement />
                <EditableElement />
            </div>
        }
    }
}

fn main() {
    //* Debuggig
    wasm_logger::init(wasm_logger::Config::default());

    //launch component
    yew::start_app::<App>();
} 
