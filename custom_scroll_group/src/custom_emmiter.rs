use event_emitter_rs::EventEmitter;
use serde::Serialize;
use serde::Deserialize;
use std::{sync::{Arc, Mutex}, thread};

#[derive(Clone)]
pub struct CustomEmmiter(Arc<Mutex<EventEmitter>>);
impl CustomEmmiter{
    pub fn new()->Self{
        CustomEmmiter(Arc::new(Mutex::new(EventEmitter::new())))
    }
    pub fn on<F, T>(&self, event: &str, callback: F) -> String
    where 
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Sync + Send 
    {
        self.0.lock().unwrap().on(event, callback)
        // let id = self.on_limited(event, None, callback);
        // return id;
    }
    pub fn emit<T>(&self, event: &str, value: T) -> Vec<thread::JoinHandle<()>>
        where T: Serialize
        {
            self.0.lock().unwrap().emit(event, value)
        }

}