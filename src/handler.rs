use std::cell::RefCell;
use std::rc::Weak;

/// The event Handler struct. This struct manages the browser events for a single type of event.
/// This crate creates a few Handler's, but only for the 'most important' browser events (currently the key events,
/// mouse events, render event and update event). Before using any of these, make sure to call the set_event_source
/// method of this crate.
/// 
/// Users can create additional Handler's to cover more events, but they will have to manually make sure that their
/// fire_event methods will be called.
/// 
/// To use a Handler, you will need to use the add_listener method. See the method description for more details.
pub struct Handler<E> {

    listeners: RefCell<Vec<Weak<RefCell<dyn Listener<E>>>>>,
    listeners_to_add: RefCell<Vec<Weak<RefCell<dyn Listener<E>>>>>
}

impl<E> Handler<E> {

    /// Creates a new event Handler. This function is *const* so that it can be used within static initializers.
    /// Once the Handler has been created, you will have to make sure that its fire_event method will be called
    /// at the right moments.
    /// 
    /// Note that you will only need this function if you need Handler's for event types that are not covered automatically
    /// by this crate. For instance, this crate already has KEY_DOWN_LISTENER, so you shouldn't need this function to
    /// create a listener for keydown events.
    pub const fn new() -> Handler<E> {
        let listeners_vec = Vec::new();
        let listeners_add_vec = Vec::new();
        Handler {
            listeners: RefCell::new(listeners_vec),
            listeners_to_add: RefCell::new(listeners_add_vec)
        }
    }

    /// Adds a listener to this Handler. The process method of all listeners of this Handler will be called when
    /// the fire_event method of this Handler is called.
    /// 
    /// It is allowed to call this method from the process method of an existing listener, in that case the new
    /// listener will be added to the list of listeners to add and will be added to the real list after all
    /// current listeners have been notified.
    /// 
    /// Note that this method requires the listener to be wrapped in a Weak<RefCell< >>, so you will need to hold
    /// the listener in an Rc<RefCell< >>. Since you provide a Weak reference to the actual listener, this method
    /// will not extend the lifetime of the listener. When the fire_event method is called and the listener is no
    /// longer alive, it will be removed from the listener list.
    /// 
    /// Currently, the only way to remove the listener is by dropping it (or let every borrow get out of scope).
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

    /// Fires the given event and notifies all (living) listeners by calling their process method. This method
    /// will also remove dead/dropped listeners from the listener list and add all listeners in the listeners
    /// to add list to the real listener list.
    /// 
    /// For the standard event Handler's of this crate like KEY_UP_HANDLER and KEY_DOWN_HANDLER, you will need
    /// to call the set_event_source method of this crate before events will start getting fired, but you won't
    /// need to call this method yourself.
    /// 
    /// For your own event Handler's, you will need to call this method manually to fire the events.
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

/// Any event listener needs to implement this trait for every event type (the type parameter) it wishes to be
/// able to listen for.
/// 
/// If a struct implements this trait for a given event, the struct can be registered as listener by calling the
/// add_listener method of a Handler for the given event.
pub trait Listener<E> {

    fn process(&mut self, event: &E);
}