//requires building with nighlty

use fltk::{app, frame::*, window::*, button::*, enums::*, group::*, prelude::*};
use comp_state::{topo, use_state, StateAccess, CloneState};

#[derive(Clone)]
pub struct Counter {
    count: StateAccess<i32>
}
pub struct GlobalEvents{}

impl GlobalEvents{
    pub const COUNTER_UPDATE: i32 = 41;
}

impl Counter {
    #[topo::nested]
    pub fn new(val: i32) -> Self {
        //initialize counter state
        let count = use_state(|| val);
        let mut pack = Pack::new(0,0,100,150, None);
        pack.set_frame(FrameType::BorderFrame);
        let mut but_inc = Button::default().with_size(0, 40).with_label("+");
        but_inc.set_color(Color::Green);


        let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label(&val.to_string());
        frame.set_frame(FrameType::FlatBox);
        frame.set_color(Color::Dark2);
        frame.set_label_color(Color::White);

        //decrement button
        let mut but_dec = Button::default().with_size(0, 40).with_label("-");
        but_dec.set_color(Color::Red);
        pack.end();

        let mut frame_cl = frame.clone();
        but_inc.set_callback(move|_|{
            count.update(move|c|{
                *c +=1;
            });
            //now that our state is updated, do the frame callback
            frame_cl.do_callback();
            //emit an event that can be globally accessed
            let _ = app::handle_main(GlobalEvents::COUNTER_UPDATE);
        });     

        
        let mut frame_cl_2 = frame.clone();
        but_dec.set_callback(move|_|{
            count.update(move|c|{
                *c -=1;
            });
            frame_cl_2.do_callback();
            let _ = app::handle_main(GlobalEvents::COUNTER_UPDATE);
        });     

        frame.set_callback(move|widg|{
            let count_val = CloneState::get(&count);
            widg.set_label(&count_val.to_string());
            widg.redraw();
        });


        Counter {
            count: count
        }

    
    }
    pub fn value(&self)->i32{
        return CloneState::get(&self.count);
    }
}


fn main() {
    let app = app::App::default();
    app::swap_frame_type(FrameType::FlatBox);

    let mut wind = Window::default().with_size(500, 300).with_label("Counter");
    let mut container = Pack::new(150,100, 200, 200, None);
    container.set_spacing(20);
    container.set_type(PackType::Horizontal);

    let c1 = Counter::new(0);
    let c2 = Counter::new(10);
    dbg!(c1.value());
    dbg!(c2.value());
    container.end();    

    let label_pack= Pack::new(100,60,300,40,"App State");
    let mut state_frame= Frame::default().with_label("Click a Counter!").with_size(300,20).with_pos(100, 20);

    //whenvera counter is clicked, update the value    
    state_frame.handle(move |widg, ev| 
      if ev == GlobalEvents::COUNTER_UPDATE.into(){
          let state_str = format!("Counter 1 Val: {}, Counter 2 Val: {}", c1.value(), c2.value());
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

