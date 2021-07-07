use std::{sync::{Mutex}, thread};
use crate::{DropObject};
use std::collections::HashMap;


pub struct DropableManager(Mutex<HashMap<String, DropObject>>);
impl DropableManager {
    pub fn new() -> Self {
        DropableManager(Mutex::new(HashMap::new()))
    }
    
    pub fn map<R>(&self, id: String, f: impl FnOnce(&DropObject) -> R) -> Option<R> {
        self.0.lock().unwrap().get(&id).map(f)
    }

    pub fn map_mut<R>(&mut self, id: String, f: impl FnOnce(&mut DropObject) -> R) -> Option<R> {
        self.0.lock().unwrap().get_mut(&id).map(f)
    }

    pub fn insert(&mut self, id: String, value: DropObject) {
        // You're taking `value` by value here, so you don't need to clone it.
        self.0.lock().unwrap().insert(id, value/*.clone()*/);
    }
}
