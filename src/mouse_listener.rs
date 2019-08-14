use super::handler::Handler;
use super::WasmuriEventSource;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::{MouseEvent,WheelEvent};

/// The MouseClickEvent is the event for the MOUSE_CLICK_HANDLER of this crate. This struct contains a single field mouse_event
/// of type web_sys::MouseEvent to make every field of the JavaScript MouseEvent available in the event. I can't simply use
/// MouseEvent as the event for the MOUSE_CLICK_HANDLER because that would make it hard to let a single listener struct listen
/// for more than 1 mouse event.
pub struct MouseClickEvent {
    pub mouse_event: MouseEvent
}

/// The MouseMoveEvent is the event for the MOUSE_MOVE_HANDLER of this crate. This struct contains a single field mouse_event
/// of type web_sys::MouseEvent to make every field of the JavaScript MouseEvent available in the event. I can't simply use
/// MouseEvent as the event for the MOUSE_MOVE_HANDLER because that would make it hard to let a single listener struct listen
/// for more than 1 mouse event.
pub struct MouseMoveEvent {
    pub mouse_event: MouseEvent
}

/// The MouseScrollEvent is the event for the MOUSE_SCROLL_HANDLER of this crate. This struct contains a single field mouse_event
/// of type web_sys::MouseEvent to make every field of the JavaScript MouseEvent available in the event. I can't simply use
/// MouseEvent as the event for the MOUSE_SCROLL_HANDLER because that would make it hard to let a single listener struct listen
/// for more than 1 mouse event.
pub struct MouseScrollEvent {
    pub mouse_event: WheelEvent
}

/// The event Handler for the JavaScript 'click' event. 
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
pub static MOUSE_CLICK_HANDLER: Handler<MouseClickEvent> = Handler::new();

/// The event Handler for the JavaScript 'mousemove' event. 
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
pub static MOUSE_MOVE_HANDLER: Handler<MouseMoveEvent> = Handler::new();

/// The event Handler for the JavaScript 'wheel' event. I do not use the 'scroll' event for this because the scroll
/// event has several annoying limitations that will prevent it from being fired in some circumstances. Unfortunately,
/// these circumstances will very likely take place in projects that rely on project wasmuri.
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
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