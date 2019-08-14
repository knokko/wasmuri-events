#![feature(const_fn,const_vec_new)]

mod handler;
mod source;

pub mod key_listener;
pub mod mouse_listener;
pub mod render_listener;
pub mod update_listener;

pub use handler::Listener;
pub use source::*;

pub fn set_event_source(source: &dyn WasmuriEventSource){
    key_listener::set_event_source(source);
    mouse_listener::set_event_source(source);
    render_listener::start_render_listener();
    update_listener::start_update_listener();
}