use yew::prelude::*;
use yew::html::{Scope};
use web_sys::{ MouseEvent, window, Element };
use web_sys::{MutationObserver, MutationObserverInit, MutationRecord};
use wasm_bindgen::{JsCast, prelude::*};
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
    ReceiveSelectedChildElement(Option<Element>),
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
    selected_child_element: Option<Element>,
    children: Vec<Html>,
    global_conds: GlobalConditions,
    global_counter: u32,
    observer_closure: Option<Closure<dyn FnMut(Vec<MutationRecord>)>>,
    observer: Option<MutationObserver>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GlobalConditions {
    is_dropzones_enabled: bool,
}
impl App {
    fn generate_observer_closure(&self, link: Scope<EditableElement>) -> Option<Closure<dyn FnMut(Vec<MutationRecord>)>> {
        let element_uuid = link.get_component().unwrap().uuid;
        let element = window().unwrap().document().unwrap().get_element_by_id(&element_uuid.to_string()).unwrap();
        let observer_closure = Closure::wrap(Box::new(move |mutations: Vec<web_sys::MutationRecord>| {
            mutations.into_iter().for_each(|mutation| {
                if mutation.attribute_name().unwrap() == "style".to_string() {
                    let new_style = element.get_attribute("style");
                    link.send_message(CMsg::SyncStyle(new_style));
                }
            });
        }) as Box<dyn FnMut(Vec<web_sys::MutationRecord>)>);
        Some(observer_closure)
    }

    fn init_observer(&self) -> Option<MutationObserver> {
        let element_uuid = self.selected_child.as_ref().unwrap().get_component().unwrap().uuid;
        let element = web_sys::window().unwrap().document().unwrap().get_element_by_id(&element_uuid.to_string()).unwrap();
        let closure = self.observer_closure.as_ref().unwrap().clone();

        let observer_res = MutationObserver::new( closure.as_ref().dyn_ref().unwrap() );
        let mut observer_init = MutationObserverInit::new();
        observer_init.attributes(true);
        let observer = observer_res.unwrap();
        observer.observe_with_options(&element, &observer_init).unwrap();
        Some(observer)
    }
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            style: CanvasStyle::create(),
            children_links: Vec::new(),
            selected_child: None,
            selected_child_element: None,
            children: vec!( html!(<EditableElement jelcount={1}></EditableElement>) ),
            global_conds: GlobalConditions {
                is_dropzones_enabled: false,
            },
            global_counter: 2,
            observer_closure: None,
            observer: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveSelectedChildLink(child_scope) => {
                // Deselect current child
                if let Some(link) = self.selected_child.as_ref() {
                    link.send_message(CMsg::Deselect);
                    self.observer.as_ref().unwrap().disconnect();
                }
                self.selected_child = Some( child_scope.clone() );
                self.observer_closure = self.generate_observer_closure(child_scope);
                self.observer = self.init_observer();
                false
            }
            Msg::ReceiveSelectedChildElement(element) => {
                self.selected_child_element = element;
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
