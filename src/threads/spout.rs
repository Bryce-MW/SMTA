use std::sync::{Condvar, Mutex};

pub struct Spout<T> {
    item: Mutex<Option<T>>,
    condvar: Condvar
}

impl<T> Spout<T> {
    pub fn new() -> Spout<T> {
        Spout {
            item: Mutex::new(None),
            condvar: Condvar::new()
        }
    }
    pub fn provide(&self, i: T) {
        // TODO(bryce): Deal with poisoned values
        let mut current = self.item.lock().unwrap();
        while !current.is_none() {
            // TODO(bryce): Deal with poisoned values
            current = self.condvar.wait(current).unwrap();
        }
        let old = current.replace(i);
        // NOTE(bryce): The condition variable should have ensured that this is true so we only
        //  confirm that it is the case on debug builds.
        debug_assert!(old.is_none());
        self.condvar.notify_one();
    }
    pub fn take(&self) -> T {
        // TODO(bryce): Deal with poisoned values
        let mut current = self.item.lock().unwrap();
        while !current.is_some() {
            // TODO(bryce): Deal with poisoned values
            current = self.condvar.wait(current).unwrap();
        }
        let old = current.take();
        // NOTE(bryce): The condition variable should have ensured that the value is always Some so
        //  this could become unchecked if needed.
        return old.unwrap();
    }
}
