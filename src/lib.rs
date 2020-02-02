#![feature(const_fn,const_vec_new)]

mod handler;
mod source;

mod key_listener;
mod mouse_listener;
mod render_listener;
mod update_listener;
mod resize_listener;
mod clipboard_listener;

pub use key_listener::*;
pub use mouse_listener::*;
pub use render_listener::*;
pub use update_listener::*;
pub use resize_listener::*;
pub use clipboard_listener::*;

pub use handler::*;
pub use source::*;

/// This function will start all event Handler's of this crate, so the handlers of this crate won't fire any events until
/// this function is called. The source can be the Window object or any HtmlElement (use the dyn_into method if you have
/// an instance of a struct that 'extends' HtmlEvent in JavaScript).
/// 
/// As the method suggests, there can only be a single event source. This function must not be called again (weird things
/// will happen if you do it anyway). This crate was simply not designed to handle events from multiple event sources
/// because the wasmuri project will operate on a single canvas anyway.
pub fn set_event_source(source: &dyn WasmuriEventSource){
    key_listener::set_event_source(source);
    mouse_listener::set_event_source(source);
    render_listener::start_render_listener();
    update_listener::start_update_listener();
    resize_listener::start_resize_listener();
    clipboard_listener::start_clipboard_listener();
}