use super::handler::Handler;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::window;

/// The UpdateEvent is the event for the UPDATE_HANDLER. It doesn't have any fields, it's only purpose is to indicate
/// that 10 milliseconds have passed since the previous update.
pub struct UpdateEvent {}

/// The event Handler for the update event (the JavaScript function that was passed to window.setInterval was called).
/// The browser will be asked to call that function every 10 milliseconds, so let's hope it will actually do it.
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
pub static UPDATE_HANDLER: Handler<UpdateEvent> = Handler::new();

pub(super) fn start_update_listener(){
    let update_closure = Closure::wrap(Box::new(|| {
        UPDATE_HANDLER.fire_event(UpdateEvent {});
    }) as Box<dyn FnMut()>);

    let window = window().expect("There should be a window");
    window.set_interval_with_callback_and_timeout_and_arguments_0(update_closure.as_ref().unchecked_ref(), 10).expect("Should be possible to call setInterval");

    update_closure.forget();
}