Custom event handling is very useful and helps in keeping custom components modular. However, using the following pattern,  we run into the issue of multiple instances of our custom widget sharing one event, leaving no way for us to handle the differently. In example below, both of our ```MyButton``` members emit the same ```MyEvent::BUTTON_CLICKED``` event, incriminating the same counter value.  

We could create a MyButton2 class with a different hard coded event value, but this defeats the purpose of having a wrapped widget in the first place.



```lang-rust
use fltk::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MyEvent;

impl MyEvent {
    const BUTTON_CLICKED: i32 = 40; // values below 30 are reserved
    const FRAME_CHANGED: i32 = 41;
}

pub struct MyButton {
    but: button::Button,
}

impl MyButton {
    pub fn new(x: i32, y: i32) -> Self {
        let mut but = button::Button::new(x, y, 80, 40, "Inc");
        but.set_callback(|_| {
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
        frame.handle(move |f, ev| {
            if ev == MyEvent::BUTTON_CLICKED.into() {
                *c.borrow_mut() += 1;
                f.set_label(&format!("{}", c.borrow()));
                let _ = app::handle_main(MyEvent::FRAME_CHANGED);
                true
            } else {
                false
            }
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
        let but = MyButton::new(100, 100);
        let but2 = MyButton::new(200, 100);
        win.show();
        win.end();
        win.handle(move |_, ev| {
            if ev == MyEvent::FRAME_CHANGED.into() {
                println!("Frame changed to value {}", frame.count());
                true
            } else {
                false
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
```
![custom_event_issue](https://user-images.githubusercontent.com/38958118/120695516-7e100c80-c479-11eb-8dd9-696221eeeb18.gif)



Lets say we have a variety of custom widgets, each with an associated custom event struct:
```rust
//my_button.rs - wrapped button widget 
struct MyButton{
    //..
}

impl MyButtonEvent { //custom events related to our button
    const BUTTON_CLICKED: i32 = 40;
    const OTHER_BUTTON_EVENT: i32 = 41;
}

//my_container.rs - wrapped container widget
struct MyContainerEvent{
    //..
}

impl MyContainerEvent[ //custom events related to our container
    const CONTAINER_ADDITION: i32 = 42; // event values can't overlap with those of MyButtonEvent
    const CONTAINER_SUBTRACTION: i32 = 43;
]

```

Since each event value is hard coded, it quickly becomes difficult keep track of our unique event ids for all of our components. I suppose we could
turn scope all our events into a single ```events``` module:
```rust
//events.rs - define event values in file to ensure no conflicting values
pub struct MyButtonEvent;

impl MyButtonEvent {
    pub const BUTTON_CLICKED: i32 = 40; // values below 30 are reserved
    pub const OTHER_BUTTON_EVENT: i32 = 41;
}

pub struct MyContainerEvent;

impl MyContainerEvent {
    pub const CONTAINER_ADDITION: i32 = 42; // values below 30 are reserved
    pub const CONTAINER_SUBTRACTION: i32 = 43;
}

//main.rs - make events accesible
pub mod events;

//button.rs - import button events
use crate::{events::MyButtonEvent};

//container.rs - import container events
use crate::{events::MyContainerEvent};
```
But I'm curious if anyone else has thoughts on an ergonomic design pattern, or a library that might help with this.




I see. In the calculator examples, buttons emit Messages which are handled inside of the same channel in ```while.app.wait()```. Is there some way to handle these events at a component level? 
```rust
enum MyComponentEvent{
    DoThing1,
    DoThing2,
}
pub struct MyComponent{
    pub fn new(&mut self, v1: v1_t)->Self{
      //..
      let (s, r) = app::channel::<MyComponentEvent>();

      while app.wait() {
        if let Some(val) = r.recv() {
            match val{
                MyComponentEvent::DoThing1=>{
                    dbg!("got 1");
                    true
                }
                MyComponentEvent::DoThing2=>{
                    dbg!("got 2");
                    true
                }
                _=>false
            }
        }
      }
      //..
    }
}
```