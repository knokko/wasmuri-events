use super::handler::Handler;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::{
    ClipboardEvent,
    window
};

pub struct CopyEvent {

    pub clipboard_event: ClipboardEvent
}

pub struct PasteEvent {

    pub clipboard_event: ClipboardEvent
}

pub struct CutEvent {

    pub clipboard_event: ClipboardEvent
}

pub static COPY_HANDLER: Handler<CopyEvent> = Handler::new();

pub static PASTE_HANDLER: Handler<PasteEvent> = Handler::new();

pub static CUT_HANDLER: Handler<CutEvent> = Handler::new();

pub(super) fn start_clipboard_listener() {

    let copy_closure = Closure::wrap(Box::new(|event| {
        COPY_HANDLER.fire_event(CopyEvent {
            clipboard_event: event
        });
    }) as Box<dyn FnMut(ClipboardEvent)>);

    let paste_closure = Closure::wrap(Box::new(|event| {
        PASTE_HANDLER.fire_event(PasteEvent {
            clipboard_event: event
        });
    }) as Box<dyn FnMut(ClipboardEvent)>);

    let cut_closure = Closure::wrap(Box::new(|event| {
        CUT_HANDLER.fire_event(CutEvent {
            clipboard_event: event
        });
    }) as Box<dyn FnMut(ClipboardEvent)>);

    let window = window().expect("There should be a window");
    window.add_event_listener_with_callback("copy", copy_closure.as_ref().unchecked_ref()).expect("Should be possible to add window copy event listener");
    window.add_event_listener_with_callback("paste", paste_closure.as_ref().unchecked_ref()).expect("Should be possible to add window paste event listener");
    window.add_event_listener_with_callback("cut", cut_closure.as_ref().unchecked_ref()).expect("Should be possible to add window cut event listener");

    copy_closure.forget();
    paste_closure.forget();
    cut_closure.forget();
}