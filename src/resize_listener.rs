use super::handler::Handler;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::window;

/// The ResizeEvent is the event of the RESIZE_HANDLER and will be fired whenever the WINDOW resizes. The struct contains
/// a get_new_width and a get_new_height method to obtain the new inner width and height of the window.
pub struct ResizeEvent {

    new_width: u32,
    new_height: u32
}

impl ResizeEvent {

    pub fn get_new_width(&self) -> u32 {
        self.new_width
    }

    pub fn get_new_height(&self) -> u32 {
        self.new_height
    }
}

/// The EventHandler for the javascript 'resize' event of the window.
/// 
/// The set_event_source method of this crate will need to be called before this Handler will start firing events, 
/// but you can always add listeners to this Handler.
pub static RESIZE_HANDLER: Handler<ResizeEvent> = Handler::new();

pub(super) fn start_resize_listener(){
    let update_closure = Closure::wrap(Box::new(|| {
        let window = window().expect("Should have window");
        RESIZE_HANDLER.fire_event(ResizeEvent {
            new_width: window.inner_width().expect("Should be able to get innerWidth of window").as_f64().expect("innerWidth of window should be an f64") as u32,
            new_height: window.inner_height().expect("Should be able to get innerHeight of window").as_f64().expect("innerHeight of window should be an f64") as u32
        });
    }) as Box<dyn FnMut()>);

    let window = window().expect("There should be a window");
    window.add_event_listener_with_callback("resize", update_closure.as_ref().unchecked_ref()).expect("Should be possible to add window resize event listener");

    update_closure.forget();
}