use std::cell::RefCell;
use std::rc::Weak;

pub(super) struct Handler<E> {

    listeners: RefCell<Vec<Weak<RefCell<dyn Listener<E>>>>>,
    listeners_to_add: RefCell<Vec<Weak<RefCell<dyn Listener<E>>>>>
}

impl<E> Handler<E> {

    pub const fn new() -> Handler<E> {
        let listeners_vec = Vec::new();
        let listeners_add_vec = Vec::new();
        Handler {
            listeners: RefCell::new(listeners_vec),
            listeners_to_add: RefCell::new(listeners_add_vec)
        }
    }

    pub fn add_listener(&self, new_listener: Weak<RefCell<dyn Listener<E>>>){

        // use try_borrow because we might be iterating over the listeners at the moment
        let maybe_borrow = self.listeners.try_borrow_mut();

        match maybe_borrow {
            Ok(mut listeners) => {
                listeners.push(new_listener);
            }, Err(_) => {

                // don't use try_borrow because listeners_to_add should not be borrowed at this stage
                let mut listeners_to_add = self.listeners_to_add.borrow_mut();
                listeners_to_add.push(new_listener);
            }
        }
    }

    pub fn fire_event(&self, event: E){

        // listeners should not be borrowed at this stage
        let mut listeners = self.listeners.borrow_mut();

        // The retain call has a double purpose, namely calling the process method of all listeners as well
        // as cleaning up the listeners that are already dropped.
        listeners.retain(|weak_listener: &Weak<RefCell<dyn Listener<E>>>| {
            let maybe_listener_cell = weak_listener.upgrade();
            match maybe_listener_cell {
                Some(listener_cell) => {
                    let mut listener = listener_cell.borrow_mut();
                    listener.process(&event);
                    true
                }, None => {
                    false
                }
            }
        });

        // It is possible that the listeners attempted to add a new listener during their process call
        // If they did, the new listeners would have been added to self.listeners_to_add.
        // So now we can add them because we have ownership of listeners at the moment.
        let mut listeners_to_add = self.listeners_to_add.borrow_mut();
        listeners.append(&mut listeners_to_add);
    }
}

// We will rely on JavaScript's single-threaded event model to make this stuff 'safe'
unsafe impl<E> Send for Handler<E> {}
unsafe impl<E> Sync for Handler<E> {}

pub trait Listener<E> {

    fn process(&mut self, event: &E);
}