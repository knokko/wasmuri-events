use js_sys::Function;

use web_sys::HtmlElement;
use web_sys::Window;

pub trait WasmuriEventSource {

    fn add_wasmuri_listener(&self, event_name: &str, handler: &Function);
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