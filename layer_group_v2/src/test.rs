// use std::{sync::{Mutex}, thread};
// use crate::{DropObject};
// use std::collections::HashMap;

// pub struct DropableManager(Mutex<HashMap<i32, String>>);
// impl DropableManager {
//     pub fn new() -> Self {
//         DropableManager(Mutex::new(HashMap::new()))
//     }
    
//     pub fn map<R>(&self, id: i32, f: impl FnOnce(&DropObject) -> R) -> Option<R> {
//         self.0.lock().unwrap().get(&id).map(f)
//     }

//     pub fn map_mut<R>(&mut self, id: i32, f: impl FnOnce(&mut DropObject) -> R) -> Option<R> {
//         self.0.lock().unwrap().get_mut(&id).map(f)
//     }

//     pub fn map_mut_pair<R>(&mut self, id1: i32, id2: i32, f: impl FnOnce(&mut DropObject, &mut DropObject) -> R){
//         assert_ne!(id1, id2, "The two keys must not resolve to the same value");
//         let a =  self.0.lock().unwrap().get_mut(&id1).unwrap() as *mut _;
//         let b =  self.0.lock().unwrap().get_mut(&id2).unwrap() as *mut _;
//         f(a, b);
//     }
//     pub fn insert(&mut self, id: i32, value: DropObject) {
//         // You're taking `value` by value here, so you don't need to clone it.
//         self.0.lock().unwrap().insert(id, value/*.clone()*/);
//     }
// }

// fn main() {
//     let mut w = HashmapWrapper::new();
//     w.insert(42, String::from("hello world"));
//     assert_eq!(w.map(42, |s| s.len()), Some(11));
//     w.map_mut(42, |s| s.push('!'));
//     assert_eq!(w.map(42, |s| s.len()), Some(12));
//     assert_eq!(w.map(42, |s| s.clone()), Some(String::from("hello world!")));
// }