use super::handler::Handler;
use super::WasmuriEventSource;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

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

    element.add_wasmuri_listener("keydown", key_down_handler.as_ref().unchecked_ref());
    element.add_wasmuri_listener("keyup", key_up_handler.as_ref().unchecked_ref());

    key_down_handler.forget();
    key_up_handler.forget();
}