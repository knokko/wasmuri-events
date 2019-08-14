#![feature(const_fn,const_vec_new)]

mod handler;
mod source;

mod key_listener;
mod mouse_listener;
mod render_listener;
mod update_listener;

pub use key_listener::*;
pub use mouse_listener::*;
pub use render_listener::*;
pub use update_listener::*;

pub use handler::*;
pub use source::*;

pub fn set_event_source(source: &dyn WasmuriEventSource){
    key_listener::set_event_source(source);
    mouse_listener::set_event_source(source);
    render_listener::start_render_listener();
    update_listener::start_update_listener();
}