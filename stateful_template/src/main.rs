use fltk::{prelude::*, *, enums::*};
use std::rc::Rc;
use std::cell::RefCell;

pub struct MyEvent;

impl MyEvent {
    const BUTTON_CLICKED: i32 = 40; // values below 30 are reserved
    const FRAME_CHANGED: i32 = 41;
}

pub struct MyButton {
    but: button::Button,
}

impl MyButton {
    pub fn new() -> Self {
        let mut but = button::Button::new(160, 200, 80, 40, "Inc");
        but.set_callback(|_| {
            dbg!("doing but cb");
            let _ = app::handle_main(MyEvent::BUTTON_CLICKED);
        });
        Self { but }
    }
}

pub struct MyFrame {
    frame: frame::Frame,
    counter: Rc<RefCell<i32>>,
}

impl MyFrame {
    pub fn new() -> Self {
        let mut frame = frame::Frame::default().size_of_parent().with_label("0");
        let counter = Rc::from(RefCell::from(0));
        let c = counter.clone();
        // frame.handle(move|f, ev|{
        //     dbg!(ev as i32);
        //     true
        // });
        frame.handle(move |f, ev|
            // te
            if ev as i32 > 30{
            // if ev as i32 == MyEvent::BUTTON_CLICKED as i32{
                *c.borrow_mut() += 1;
                dbg!("doing event");
                f.set_label(&format!("{}", c.borrow()));
                // std::thread::spawn(move||{
                    let _ = app::handle_main(MyEvent::FRAME_CHANGED);
                // });
                dbg!("got here!");
            true
        } else {
            false
        });
        Self { frame, counter }
    }
    pub fn count(&self) -> i32 {
        *self.counter.borrow()
    }
}

pub struct MyWindow {
    win: window::Window,
}

impl MyWindow {
    pub fn new() -> Self {
        let mut win = window::Window::default().with_size(400, 300);
        let frame = MyFrame::new();
        let but = MyButton::new();
        win.show();
        win.end();
        win.handle(move |_, ev| if ev as i32 == MyEvent::FRAME_CHANGED {
            println!("Frame changed to value {}", frame.count());
            true
        } else {
            match ev {
                Event::Push=>{
                    dbg!("got pressed");
                    true
                }
                _=>false
            }
        });
        Self { win }
    }
}

fn main() {
    let app = app::App::default();
    let _win = MyWindow::new();
    app.run().unwrap();
}