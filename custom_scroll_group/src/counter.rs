use std::{sync::{Arc, Mutex}};
use std::ops::{Add, Mul, Sub};

#[derive(Clone)]
pub struct Counter<T>(Arc<Mutex<T>>);

impl<T: Copy> Counter<T>{
    
    pub fn new(val:T) -> Self {
        Counter(Arc::new(Mutex::new(val)))
    }
    pub fn increment(&self, by: T)  where T: Add<Output =T>{
        let mut counter = self.0.lock().unwrap();
        *counter = *counter + by;
    }
    pub fn decrement(&self, by: T)  where T: Sub<Output=T> {
        let mut counter = self.0.lock().unwrap();
        *counter = *counter - by;
    }
    pub fn multiply(&self, by: T) where T: Mul<Output=T> {
        let mut counter = self.0.lock().unwrap();
        *counter = *counter * by;
    }
    pub fn get(&self) -> T {
        let counter = self.0.lock().unwrap();
        *counter
    }
    pub fn set(&self, val: T){
        let mut counter = self.0.lock().unwrap();
        *counter = val
    }
}