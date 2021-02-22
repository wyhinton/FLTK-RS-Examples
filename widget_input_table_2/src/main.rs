//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*, valuator::*};
// use input_table;

pub mod input_group;
use input_group::InputGroup;  
use input_group::base_mutex_valuator::*; 
// use 
use std::fmt;
use uuid::Uuid; 
pub trait MValExt{} 

#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}


#[derive(Clone, Debug)]
pub enum MutexValuator{
    IntInput,
    FloatInput,
    DialInput, 
}


#[derive(Clone)]
pub struct InputConfig<T = f64>{
    pub id: Uuid,
    pub val_name: String, 
    pub val: T, 
    pub inp_type: MutexValuator,
    pub bounds: (T, T),
    pub step: T,
}


//default input config is a float slider
impl Default for InputConfig{
    fn default() -> InputConfig {
        InputConfig{
            id: Uuid::new_v4(),
            val_name: "Test Input Name".to_string(), 
            val: 0.0, 
            inp_type: MutexValuator::FloatInput,
            bounds: (0.0, 250.0),
            step: 1.0,
        }
    }
}


impl fmt::Debug for InputConfig{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value Name: {}, value: {}, Input Type: {:?}, bounds: {},{} step: {} ", self.val_name, self.val, self.inp_type, self.bounds.0, self.bounds.1, self.step)
    }
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    win.make_resizable(true);
    
    let t_width = 200;
    
    let input_panel = InputPanel::new(200,100,500,500);
    
    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    // println!("{}", "got test message");
                    app::redraw();
                }
            }
        }
    }
}

