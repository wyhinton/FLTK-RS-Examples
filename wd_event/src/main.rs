

// use fltk::{app, app::*, group::Pack, group::*, self, button::*, draw, enums::*, frame::*, prelude::*, widget::*, window::*};
// use comp_state::{topo, use_state, use_state_current, StateAccess, CloneState};

// pub struct Counter{
// }

// impl Counter{
//     #[topo::nested]
//     pub fn new(x: i32, y: i32, w: i32, h: i32)->Self{
//         let mut inc_button= Button::new(x,y,w,h,"inc");
//         inc_button.set_callback(move|widg|{
          
//         });
//         Counter{
            
//         }
//     }
// }

// fn main() {
//     let app = App::default();
//     let mut win = Window::new(200, 200, 1000, 1000, "Template");
//     let c1 = Counter::new(200,200,200,200);
//     win.end();
//     win.show();
//     app.run().unwrap();
// }