//example of using the event emmiter crate
use fltk::{app, enums::FrameType, frame::Frame, prelude::*, window::Window, button::*, group::*, enums::*};
use std::{sync::{Arc, Mutex}};
use event_emitter_rs::EventEmitter;
use lazy_static::lazy_static;

//Globally accessible event
lazy_static! {
    pub static ref GLOBAL_EVENT_EMMITER: Mutex<EventEmitter> = Mutex::new(EventEmitter::new());
}

struct GlobalEVents{
}

impl GlobalEVents{
    const SET_GLOBAL_FRAME: &'static str = "SET_GLOBAL_FRAME";
}
struct MyButton{
}

impl MyButton{
    pub fn new(x: i32, y: i32, w: i32, h: i32, event: &'static str, emmitter: Arc<Mutex<EventEmitter>>, button_val: i32)->Self{
        let mut button= Button::new(x,y,w,h,None);
        button.set_frame(FrameType::FlatBox);
        button.set_label(&format!("+{}", button_val));
        button.set_callback(move|_|{
            emmitter.lock().unwrap().emit(event, button_val);
        });
        MyButton{
        }
    }
}

//Utility wrapper struct allows us to mutate a i32 value
#[derive(Clone)]
struct ConcurrentCounter(Arc<Mutex<i32>>);

impl ConcurrentCounter{
    
pub fn new(val: i32) -> Self {
    ConcurrentCounter(Arc::new(Mutex::new(val)))
}
pub fn increment(&self, by: i32) {
    let mut counter = self.0.lock().unwrap();
    *counter = *counter + by;
}
pub fn get(&self) -> i32 {
    let counter = self.0.lock().unwrap();
    *counter
}
}

struct CounterGroup{}

struct AdderEvents{}
impl AdderEvents{
    const ADD_VALUE: &'static str = "ADD_VALUE";
}

impl CounterGroup{
    pub fn new(x: i32, y: i32, w: i32, h: i32)->Self{
        //wrap our emmitter so we can clone and use it everywhere inside Adder
        let event_emitter = Arc::new(Mutex::new(EventEmitter::new()));
        //i32 value accesible from within Adder
        let counter = ConcurrentCounter::new(0);
        //displays our i32 value, which we will be mutating via emmisions from our MyButton structs
        let display_frame= Arc::new(Mutex::new(Frame::new(x+w+50,y,200,20,"0")));

        let counter_cl = counter.clone();
        let disp_frame_cl = display_frame.clone();
        //describes how to handle an internal event
        event_emitter.lock().unwrap().on(AdderEvents::ADD_VALUE, move |number: i32| 
        {
            counter_cl.increment(number);
            disp_frame_cl.lock().unwrap().set_label(&counter_cl.get().to_string());
            dbg!(number);
        });

        let container= Pack::new(x,y,w,h, None);
        //Other instances emit predfined events
        let _myb2 = MyButton::new(100,100,50,20, AdderEvents::ADD_VALUE,  event_emitter.clone(), 10);
        let _myb3 = MyButton::new(100,100,50,20, AdderEvents::ADD_VALUE,  event_emitter.clone(), 20);
        container.end();    
        CounterGroup{}
    }
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 500, 500, "Hello from rust");

    //displays a string value which is mutated via a Global Event 
    let global_frame = Arc::new(Mutex::new(Frame::new(100,50,200,20,"myframe")));
    global_frame.lock().unwrap().set_color(Color::Red);
    global_frame.lock().unwrap().set_frame(FrameType::FlatBox);

    //Handle our the global event
    GLOBAL_EVENT_EMMITER.lock().unwrap().on(GlobalEVents::SET_GLOBAL_FRAME, move|frame_label: String|{
        global_frame.lock().unwrap().set_label(&frame_label);
    });
    
    //Custom adder widget can listen to events internally and global,
    let _parent = CounterGroup::new(100,100,100,50);
    let _parent2 = CounterGroup::new(100,200,100,50);

    wind.make_resizable(true);
    wind.end();
    wind.show();
    //No event need handling in main event loop
    app.run().unwrap();
}