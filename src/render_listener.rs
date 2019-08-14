use std::cell::RefCell;
use std::rc::Weak;

use super::handler::Handler;
use super::handler::Listener;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::window;

pub struct RenderEvent {}

static RENDER_HANDLER: Handler<RenderEvent> = Handler::new();

pub fn add_render_listener(listener: Weak<RefCell<dyn Listener<RenderEvent>>>){
    RENDER_HANDLER.add_listener(listener);
}

pub fn start_render_listener(){
    request_next_frame();
}

fn request_next_frame(){
    let render_closure = Closure::once_into_js(Box::new(|| {
        RENDER_HANDLER.fire_event(RenderEvent {});
        request_next_frame();
    }) as Box<dyn FnOnce()>);

    let window = window().expect("There should be a window");
    window.request_animation_frame(render_closure.as_ref().unchecked_ref()).expect("Should be possible to request an animation frame");
}