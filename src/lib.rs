#![feature(const_fn,const_vec_new)]

mod handler;
mod source;

pub mod key_listener;
pub mod mouse_listener;

pub use handler::Listener;
pub use source::*;

pub fn add_event_source(source: &dyn WasmuriEventSource){
    key_listener::add_event_source(source);
    mouse_listener::add_event_source(source);
}