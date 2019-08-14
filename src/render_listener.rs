use super::handler::Handler;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::window;

/// The RenderEvent is the event for the RENDER_HANDLER of this crate. It doesn't contain any fields, it's only
/// purpose is to indicate that the browser has given an animation frame.
pub struct RenderEvent {}

/// The RENDER_HANDLER is the event Handler for the animation frame event (the JavaScript function that was passed to
/// window.requestAnimationFrame is being called). 
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
pub static RENDER_HANDLER: Handler<RenderEvent> = Handler::new();

pub(super) fn start_render_listener(){
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