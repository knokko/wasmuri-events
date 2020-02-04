use super::handler::Handler;
use super::*;

use web_sys::KeyboardEvent;

/// The event Handler for the 'keydown' JavaScript event. 
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
pub static KEY_DOWN_HANDLER: Handler<KeyDownEvent> = Handler::new();

/// The event Handler for the 'keyup' JavaScript event. 
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
pub static KEY_UP_HANDLER: Handler<KeyUpEvent> = Handler::new();

/// The KeyDownEvent is the event of the KEY_DOWN_HANDLER. It contains a single public field key_event of type
/// KeyboardEvent. This struct is made that way so that all properties of the JavaScript KeyboardEvent can be
/// accessed while allowing a single struct to implement both Listener<KeyDownEvent> and Listener<KeyUpEvent>.
pub struct KeyDownEvent {
    
    pub key_event: KeyboardEvent
}

/// The KeyUpEvent is the event of the KEY_UP_HANDLER. It contains a single public field key_event of type
/// KeyboardEvent. This struct is made that way so that all properties of the JavaScript KeyboardEvent can be
/// accessed while allowing a single struct to implement both Listener<KeyDownEvent> and Listener<KeyUpEvent>.
pub struct KeyUpEvent {
    
    pub key_event: KeyboardEvent
}

pub(super) fn set_event_source(element: &dyn WasmuriEventSource){
    add_wasmuri_listener(element, &KEY_DOWN_HANDLER, |key_event| KeyDownEvent{key_event}, "keydown");
    add_wasmuri_listener(element, &KEY_UP_HANDLER, |key_event| KeyUpEvent{key_event}, "keyup");
}