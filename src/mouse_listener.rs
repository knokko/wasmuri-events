use super::*;

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

impl MouseMoveEvent {

    pub fn get_new_position(&self) -> (i32, i32) {
        (self.mouse_event.offset_x(), self.mouse_event.offset_y())
    }
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
    add_wasmuri_listener(element, &MOUSE_CLICK_HANDLER, |mouse_event| MouseClickEvent{mouse_event}, "click");
    add_wasmuri_listener(element, &MOUSE_MOVE_HANDLER, |mouse_event| MouseMoveEvent{mouse_event}, "mousemove");
    add_wasmuri_listener(element, &MOUSE_SCROLL_HANDLER, |mouse_event| MouseScrollEvent{mouse_event}, "wheel");
}