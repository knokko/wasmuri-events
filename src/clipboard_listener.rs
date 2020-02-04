use super::handler::Handler;
use super::source::add_window_listener;

use web_sys::ClipboardEvent;

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
    add_window_listener(&COPY_HANDLER, |clipboard_event| CopyEvent {clipboard_event}, "copy");
    add_window_listener(&PASTE_HANDLER, |clipboard_event| PasteEvent {clipboard_event}, "paste");
    add_window_listener(&CUT_HANDLER, |clipboard_event| CutEvent {clipboard_event}, "cut");
}