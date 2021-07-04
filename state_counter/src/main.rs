use fltk::{app, button::Button, group::*, frame::Frame, enums::*, group::Pack, prelude::*, window::Window};
use state::Storage;
use std::collections::HashMap;
use std::sync::Mutex;

//convience macro for extracting value of an enum
#[macro_use]
macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

//our global hashamp for storing widget ids and their associated values
static GLOBAL_MAP: Storage<Mutex<HashMap<String, WidgetValue>>> = Storage::new();


// enum wrapper for differnet value types allows our global map to store multiple types
#[derive(Clone, Copy)]
pub enum WidgetValue{
    Integer32(i32),
    Unsized32(u32)
}

#[derive(Clone)]
pub struct Counter {
    count: Storage<i32>
}

//for a simple example we'll use global events and app::handle_main to trigger updates outside of the wrapped widget
pub struct GlobalEvents{}

impl GlobalEvents{
    pub const COUNTER_UPDATE: i32 = 41;
}

impl Counter {
    pub fn new(val: i32, id: &'static str) -> Self {
        //insert counters value into the global hashmap, using it's id as a key
        let mut map = GLOBAL_MAP.get().lock().unwrap();
        map.insert(id.into(),  WidgetValue::Integer32(0));


        let mut pack = Pack::new(0,0,100,150, None);
        pack.set_frame(FrameType::BorderFrame);
        //increment button
        let mut but_inc = Button::default().with_size(0, 40).with_label("+");
        but_inc.set_color(Color::Green);

        //frame for displaying current counter
        //fetch counter starting value
        let display_label = match map.get(id.into()).unwrap(){
            WidgetValue::Integer32(val)=>{
                let my_val = val.to_string();
                my_val
            }
            _=>("no value".to_string())
        };
        let mut frame = Frame::default()
            .with_size(0, 40)
            .with_label(&display_label.to_string());
        frame.set_frame(FrameType::FlatBox);
        frame.set_color(Color::Dark2);
        frame.set_label_color(Color::White);

        //decrement button
        let mut but_dec = Button::default().with_size(0, 40).with_label("-");
        but_dec.set_color(Color::Red);

        pack.end();

        //increment value, update frame label
        let mut frame_cl = frame.clone();
        but_inc.set_callback(move|_|{
            let enum_innner_val = *GLOBAL_MAP.get().lock().unwrap().get(id).unwrap();
            use WidgetValue::*;
            let inner_value = match enum_innner_val {
                Integer32(val)=>{
                    val
                },
                _=>(0)
            };
            frame_cl.set_label(&inner_value.to_string());
            *GLOBAL_MAP.get().lock().unwrap().get_mut(id).unwrap() = Integer32(inner_value+1);
            let _ = app::handle_main(GlobalEvents::COUNTER_UPDATE);
        });

        //decrement value, update frame label
        let mut frame_cl_2 = frame.clone();
        // let mut frame_cl_2 = Rc::clone(frame);
        but_dec.set_callback(move|_|{
            let enum_innner_val = GLOBAL_MAP.get().lock().unwrap().get(id).unwrap().clone();
            use WidgetValue::*;
            let inner_value = match enum_innner_val {
                Integer32(val)=>{
                    val
                },
                _=>(0)
            };
            frame_cl_2.set_label(&inner_value.to_string());
            *GLOBAL_MAP.get().lock().unwrap().get_mut(id).unwrap() = Integer32(inner_value-1);
            let _ = app::handle_main(GlobalEvents::COUNTER_UPDATE);
        });
    
        Counter {
            count: Storage::from(val)
        }
    
    }
}

fn main(){
    //initialize store
    let initial_map = HashMap::new();
    GLOBAL_MAP.set(Mutex::new(initial_map));

    let app = app::App::default();
    app::swap_frame_type(FrameType::FlatBox);

    let mut wind = Window::default().with_size(500, 300).with_label("Counter");
    let mut container = Pack::new(150,100, 200, 200, None);
    container.set_spacing(20);
    container.set_type(PackType::Horizontal);

    let _c1 = Counter::new(0, "Counter 1");
    let _c2 = Counter::new(10, "Counter 2");
    container.end();    
    
    let label_pack= Pack::new(100,60,300,40,"App State");
    let mut state_frame= Frame::default().with_label("Click a Counter!").with_size(300,20).with_pos(100, 20);

    //update our "App State" frame whenever the counter buttons are presssed
    state_frame.handle(move |widg, ev| 
      if ev == GlobalEvents::COUNTER_UPDATE.into(){
        let mut counter_1_val = 0;
        if let Some(WidgetValue::Integer32(val)) = GLOBAL_MAP.get().lock().unwrap().get("Counter 1"){
        counter_1_val = *val;
        }
        let mut counter_2_val = 0;
        if let Some(WidgetValue::Integer32(val)) = GLOBAL_MAP.get().lock().unwrap().get("Counter 2"){
        counter_2_val = *val;
        }
        //   dbg!(val);
        //   let counter_1_val = as_variant!(*GLOBAL_MAP.get().lock().unwrap().get("Counter 1").unwrap(), WidgetValue::Integer32).expect("failed to get int");
        //   let counter_2_val = as_variant!(*GLOBAL_MAP.get().lock().unwrap().get("Counter 2").unwrap(), WidgetValue::Integer32).expect("failed to get int");
          let state_str = format!("Counter 1 Val: {}, Counter 2 Val: {}", counter_1_val, counter_2_val);
          widg.set_damage(true);
          widg.set_label(&state_str);
        true
    } else {
        false
    });
    state_frame.set_frame(FrameType::FlatBox);
    state_frame.set_color(Color::Green);
    label_pack.end();
    wind.end();
    wind.show();
    app.run().unwrap();
}


