use std::cell::RefCell;
use std::rc::Weak;

use super::handler::Handler;
use super::handler::Listener;
use super::WasmuriEventSource;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use wasmuri_core::util::print;

use web_sys::MouseEvent;

pub struct MouseClickEvent {
    pub mouse_event: MouseEvent
}

pub struct MouseMoveEvent {
    pub mouse_event: MouseEvent
}

static MOUSE_CLICK_HANDLER: Handler<MouseClickEvent> = Handler::new();

static MOUSE_MOVE_HANDLER: Handler<MouseMoveEvent> = Handler::new();

pub fn add_mouse_click_listener(listener: Weak<RefCell<dyn Listener<MouseClickEvent>>>){
    MOUSE_CLICK_HANDLER.add_listener(listener);
    print("Added mouse click listener");
}

pub fn add_mouse_move_listener(listener: Weak<RefCell<dyn Listener<MouseMoveEvent>>>){
    MOUSE_MOVE_HANDLER.add_listener(listener);
    print("Added mouse move listener");
}

pub fn add_event_source(element: &dyn WasmuriEventSource){
    print("Listen mouse on something");
    let mouse_click_handler = Closure::wrap(Box::new(|event: MouseEvent| {
        print("Fired mouse click event");
        MOUSE_CLICK_HANDLER.fire_event(MouseClickEvent {
            mouse_event: event
        });
    }) as Box<dyn FnMut(MouseEvent)>);
    let mouse_move_handler = Closure::wrap(Box::new(|event: MouseEvent| {
        print("Fired mouse move event");
        MOUSE_MOVE_HANDLER.fire_event(MouseMoveEvent {
            mouse_event: event
        });
    }) as Box<dyn FnMut(MouseEvent)>);

    element.add_wasmuri_listener("click", mouse_click_handler.as_ref().unchecked_ref());
    element.add_wasmuri_listener("mousemove", mouse_move_handler.as_ref().unchecked_ref());

    mouse_click_handler.forget();
    mouse_move_handler.forget();
}