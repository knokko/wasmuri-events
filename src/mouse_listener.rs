use super::handler::Handler;
use super::WasmuriEventSource;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::{MouseEvent,WheelEvent};

pub struct MouseClickEvent {
    pub mouse_event: MouseEvent
}

pub struct MouseMoveEvent {
    pub mouse_event: MouseEvent
}

pub struct MouseScrollEvent {
    pub mouse_event: WheelEvent
}

pub static MOUSE_CLICK_HANDLER: Handler<MouseClickEvent> = Handler::new();

pub static MOUSE_MOVE_HANDLER: Handler<MouseMoveEvent> = Handler::new();

pub static MOUSE_SCROLL_HANDLER: Handler<MouseScrollEvent> = Handler::new();

pub(super) fn set_event_source(element: &dyn WasmuriEventSource){
    let mouse_click_handler = Closure::wrap(Box::new(|event: MouseEvent| {
        MOUSE_CLICK_HANDLER.fire_event(MouseClickEvent {
            mouse_event: event
        });
    }) as Box<dyn FnMut(MouseEvent)>);
    let mouse_move_handler = Closure::wrap(Box::new(|event: MouseEvent| {
        MOUSE_MOVE_HANDLER.fire_event(MouseMoveEvent {
            mouse_event: event
        });
    }) as Box<dyn FnMut(MouseEvent)>);
    let mouse_scroll_handler = Closure::wrap(Box::new(|event: WheelEvent| {
        MOUSE_SCROLL_HANDLER.fire_event(MouseScrollEvent {
            mouse_event: event
        });
    }) as Box<dyn FnMut(WheelEvent)>);

    element.add_wasmuri_listener("click", mouse_click_handler.as_ref().unchecked_ref());
    element.add_wasmuri_listener("mousemove", mouse_move_handler.as_ref().unchecked_ref());
    element.add_wasmuri_listener("wheel", mouse_scroll_handler.as_ref().unchecked_ref());

    mouse_click_handler.forget();
    mouse_move_handler.forget();
    mouse_scroll_handler.forget();
}