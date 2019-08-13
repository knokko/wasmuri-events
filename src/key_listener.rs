use std::cell::RefCell;
use std::rc::Weak;

use super::handler::Handler;
use super::handler::Listener;
use super::WasmuriEventSource;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use wasmuri_core::util::print;

use web_sys::KeyboardEvent;

static KEY_DOWN_HANDLER: Handler<KeyDownEvent> = Handler::new();

static KEY_UP_HANDLER: Handler<KeyUpEvent> = Handler::new();

pub struct KeyDownEvent {
    
    pub key_event: KeyboardEvent
}

pub struct KeyUpEvent {
    
    pub key_event: KeyboardEvent
}

pub fn add_key_down_listener(listener: Weak<RefCell<dyn Listener<KeyDownEvent>>>){
    KEY_DOWN_HANDLER.add_listener(listener);
    print("Added key down listener");
}

pub fn add_key_up_listener(listener: Weak<RefCell<dyn Listener<KeyUpEvent>>>){
    KEY_UP_HANDLER.add_listener(listener);
    print("Added key up listener");
}

pub fn add_event_source(element: &dyn WasmuriEventSource){
    print("Listen key on something");
    let key_down_handler = Closure::wrap(Box::new(|event: KeyboardEvent| {
        print("Fired key down event");
        KEY_DOWN_HANDLER.fire_event(KeyDownEvent {
            key_event: event
        });
    }) as Box<dyn FnMut(KeyboardEvent)>);
    let key_up_handler = Closure::wrap(Box::new(|event: KeyboardEvent| {
        print("Fired key up event");
        KEY_UP_HANDLER.fire_event(KeyUpEvent {
            key_event: event
        });
    }) as Box<dyn FnMut(KeyboardEvent)>);

    element.add_wasmuri_listener("keydown", key_down_handler.as_ref().unchecked_ref());
    element.add_wasmuri_listener("keyup", key_up_handler.as_ref().unchecked_ref());

    key_down_handler.forget();
    key_up_handler.forget();
}