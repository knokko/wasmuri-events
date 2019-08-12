use std::cell::RefCell;
use std::rc::Weak;

use super::handler::Handler;
use super::handler::Listener;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::Window;
use web_sys::KeyboardEvent;
use web_sys::window;

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
}

pub fn add_key_up_listener(listener: Weak<RefCell<dyn Listener<KeyUpEvent>>>){
    KEY_UP_HANDLER.add_listener(listener);
}

#[wasm_bindgen(start)]
pub fn start_listening(){

    listen_on(window().expect("Should have window"));
}

fn listen_on(element: Window){
    let key_down_handler = Closure::wrap(Box::new(|event: KeyboardEvent| {
        KEY_DOWN_HANDLER.fire_event(KeyDownEvent {
            key_event: event
        });
    }) as Box<dyn FnMut(KeyboardEvent)>);
    let key_up_handler = Closure::wrap(Box::new(|event: KeyboardEvent| {
        KEY_UP_HANDLER.fire_event(KeyUpEvent {
            key_event: event
        });
    }) as Box<dyn FnMut(KeyboardEvent)>);

    element.add_event_listener_with_callback("keydown", key_down_handler.as_ref().unchecked_ref()).unwrap();
    element.add_event_listener_with_callback("keyup", key_up_handler.as_ref().unchecked_ref()).unwrap();

    key_down_handler.forget();
    key_up_handler.forget();
}