//ordinal events example
/// using app::handle_main can result in more ergonomic code, allowing any widget to handle the event, keeping ```main``` more organized. 
/// However, custom event values are hard coded. 
/// enum_ordanlize let us define our custom event values relative to the number of total enum values. Thus we can
/// insert or remove custom events without worrying about disjoint values.

use fltk::{app, app::*, frame::*, window::*, button::*, prelude::*, group::*};
use enum_ordinalize::Ordinalize;
use std::rc::Rc;
use std::cell::RefCell;

 //specify i32 for our custom event is in i32, not i8 (the default)
#[derive(Debug, PartialEq, Eq, Ordinalize)]
#[repr(i32)]
enum CustomEvents{
    AddOne = 41,
    AddTwo,
    AddThree,
}
struct Adder{
    pack: Pack
}

impl Adder{
    pub fn new()->Self{
        let mut container= Pack::new(80,50,200,20,"Adder Widget");
        let mut button= Button::new(0,0,50,20,"add1");
        button.set_callback(move|widg|{
          let _ = app::handle_main(CustomEvents::AddOne.ordinal());
        });
        let mut button2= Button::new(0,0,50,20,"add2");
        button2.set_callback(move|widg|{
          let _ = app::handle_main(CustomEvents::AddTwo.ordinal());
        });
        let mut button3= Button::new(0,0,50,20,"add3");
        button3.set_callback(move|widg|{
          let _ = app::handle_main(CustomEvents::AddThree.ordinal());
        });
        container.end();
        container.set_type(PackType::Horizontal);
        Adder{
            pack: container
        }
    }
}

struct MyWindow{
  
}

impl MyWindow{
    pub fn new()->Self{
        let counter = Rc::from(RefCell::from(0));
        
        let mut win = Window::new(200, 200, 300, 200, "Ordinal Events");
        let mut disp_frame= Frame::new(200,0,200,200,"0").center_of_parent();
        let _adder = Adder::new();

        let counter_cl = counter.clone();
        //update our frame's label with the counter value
        disp_frame.handle(move |widg, ev| 
            if ev.bits() == CustomEvents::AddOne.ordinal(){
            widg.set_label(&*counter_cl.borrow_mut().to_string());
            dbg!("also handled event here");
            true
          } else if ev.bits() == CustomEvents::AddTwo.ordinal(){
            widg.set_label(&*counter_cl.borrow_mut().to_string());
            dbg!("also handled event here");
            true
          } else if ev.bits() == CustomEvents::AddThree.ordinal(){
            widg.set_label(&*counter_cl.borrow_mut().to_string());
            dbg!("also handled event here");
            true
          } else {
            false
          });

        win.end();
        win.show();

        //increment the counter value
        win.handle(move |_, ev| 
          if ev.bits() == CustomEvents::AddOne.ordinal(){
            *counter.borrow_mut() += 1;
            true
        } else if ev.bits() == CustomEvents::AddTwo.ordinal(){
            *counter.borrow_mut() += 2;
            true
        } else if ev.bits() == CustomEvents::AddThree.ordinal(){
            *counter.borrow_mut() += 3;
            true
        } else {
            false
        });
        
        MyWindow{}
    }
}
fn main() {
    let app = App::default();
    let _ = MyWindow::new();
    dbg!(CustomEvents::AddOne.ordinal());
    dbg!(CustomEvents::AddTwo.ordinal());
    dbg!(CustomEvents::AddThree.ordinal());
// [src\main.rs:126] CustomEvents::AddOne.ordinal() = 41
// [src\main.rs:127] CustomEvents::AddTwo.ordinal() = 42
// [src\main.rs:128] CustomEvents::AddThree.ordinal() = 43
    app.run().unwrap();
}

