use std::cell::RefCell;
use std::rc::Weak;

use super::handler::Handler;
use super::handler::Listener;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::window;

pub struct UpdateEvent {}

static UPDATE_HANDLER: Handler<UpdateEvent> = Handler::new();

pub fn add_update_listener(listener: Weak<RefCell<dyn Listener<UpdateEvent>>>){
    UPDATE_HANDLER.add_listener(listener);
}

pub fn start_update_listener(){
    let update_closure = Closure::wrap(Box::new(|| {
        UPDATE_HANDLER.fire_event(UpdateEvent {});
    }) as Box<dyn FnMut()>);

    let window = window().expect("There should be a window");
    window.set_interval_with_callback_and_timeout_and_arguments_0(update_closure.as_ref().unchecked_ref(), 10).expect("Should be possible to call setInterval");

    update_closure.forget();
}