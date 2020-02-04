use super::*;

use wasm_bindgen::JsValue;

use web_sys::Event;
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
    let window = window().expect("There should be a window");
    add_window_listener(&RESIZE_HANDLER, move |_event: Event| ResizeEvent {
        new_width: convert(window.inner_width()),
        new_height: convert(window.inner_height())
    }, "resize");
}

fn convert(value: Result<JsValue,JsValue>) -> u32 {
    value.expect("window should have innerWidth and innerHeight").as_f64().expect(
            "window.innerWidth and window.innerHeight should be floating point numbers") as u32
}