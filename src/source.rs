use crate::Handler;
use js_sys::Function;

use wasm_bindgen::JsCast;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;

use web_sys::HtmlElement;
use web_sys::window;
use web_sys::Window;

/// A WasmuriEventSource is a JavaScript object that can have event listeners. A reference to this trait will need to be
/// passed to the set_event_source function of this crate.
/// 
/// Ideally, I would let the Rust equivalent of every such JavaScript object implement this trait, but that would be a
/// lot of work since web-sys has many such structs. That's why currently only HtmlElement and Window implement this trait.
/// 
/// This should be sufficient since you can use the dyn_into method of many such structs to convert it to a HtmlElement.
pub trait WasmuriEventSource {

    /// This should basically call the addEventListener of the JavaScript object, which is currently usually done by
    /// invoking the add_event_listener_with_callback on the Rust equivalent.
    fn add_wasmuri_listener(&self, event_name: &str, handler: &Function);
}

pub fn add_window_listener<E,H: FromWasmAbi + 'static,F : Fn(H) -> E + 'static>(event_handler: &'static Handler<E>, f: F, event_name: &str) {
    add_wasmuri_listener(&window().expect("There should be a window"), event_handler, f, event_name);
}

pub fn add_wasmuri_listener<E,H: FromWasmAbi + 'static,F : Fn(H) -> E + 'static>(element: &dyn WasmuriEventSource, event_handler: &'static Handler<E>, f: F, event_name: &str) {
    let closure = Closure::wrap(Box::new(move |event| {
        event_handler.fire_event(f(event));
    }) as Box<dyn FnMut(H)>);

    element.add_wasmuri_listener(event_name, closure.as_ref().unchecked_ref());

    closure.forget();
}

impl WasmuriEventSource for HtmlElement {

    fn add_wasmuri_listener(&self, event_name: &str, handler: &Function){
        self.add_event_listener_with_callback(event_name, handler).expect("Should be able to add event listener");
    }
}

impl WasmuriEventSource for Window {

    fn add_wasmuri_listener(&self, event_name: &str, handler: &Function){
        self.add_event_listener_with_callback(event_name, handler).expect("Should be able to add event listener");
    }
}