use fltk::prelude::*;
use fltk::*;
use fltk::{app::*, button::*, enums::*, group::*, window::*};
pub mod scroll_group;
use fltk::{prelude::GroupExt, prelude::WidgetExt};
use scroll_group::ScrollGroup;
use scroll_group::ScrollGroupEvent;
use scroll_group::CustomScrollEvent;
pub mod scroll_bar;
use scroll_bar::ScrollBar;
pub mod events;
pub mod custom_emmiter;
pub mod counter;
use counter::Counter;
use custom_emmiter::CustomEmmiter;
pub mod arc_widget;
use arc_widget::ArcWidget;
use arc_widget::ArcGroup;
use std::{sync::{Arc, Mutex}};
use event_emitter_rs::EventEmitter;
use serde::{Serialize, Deserialize};
pub mod button;
use button::MyButton;
// use comp_state::{topo, use_state};

// pub mod scroll

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct WidgDimensions{
  pub width: i32,
  pub height: i32,
}

impl WidgDimensions{
    pub fn new(w: i32, h: i32)->Self{
        WidgDimensions{
            width: w,
            height: h,
        }
    }
}
pub struct ButtonEvent;

impl ButtonEvent {
    pub const ADD_BUTTON_PUSH: i32 = 43; // values below 30 are reserved
    pub const SUBTRACT_BUTTON_PUSH: i32 = 44;
}
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_EVENT_EMMITER: Mutex<EventEmitter> = Mutex::new(EventEmitter::new());
}


pub struct GlobalEvent{}
impl GlobalEvent{
    pub const ADD_BUTTON: &'static str = "ADD_BUTTON";
    pub const REMOVE_BUTTON: &'static str = "REMOVE_BUTTON";
}


#[derive(Debug, Clone)]
pub enum Message {
    Test,
    AddButton,
    RemoveButton,
}

fn main() {
    let sg_width = 300;

    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    win.make_resizable(true);
    let mut button_container = Pack::new(350, 150, sg_width, 50, None);

    //BUTTONS
    let mut add_button = Button::new(0, 0, 150, 50, "Add Button");
    add_button.set_callback(move |_| {
        GLOBAL_EVENT_EMMITER.lock().unwrap().emit(GlobalEvent::ADD_BUTTON, 1);
    });
    let mut subtract_button = Button::new(0, 0, 150, 50, "Subtract Button");
    subtract_button.set_callback(move |_| {
        GLOBAL_EVENT_EMMITER.lock().unwrap().emit(GlobalEvent::REMOVE_BUTTON, 1);
    });
    button_container.end();
    button_container.set_type(PackType::Horizontal);

    //FIRST SCROLL GROUP
    let mut test_pack = Pack::new(100, 500, sg_width, 1000, "");
    test_pack.end();

    for x in 0..20 {
        let mut but = Button::default()
            .with_size(100, 50)
            .with_label(&x.to_string());
        but.set_color(Color::Red);
        test_pack.add(&but);
    }
    dbg!(test_pack.children());
    test_pack.redraw();
    // dbg!(test_pack.height());
    let mut sg = ScrollGroup::new(
        (win.width() - sg_width-400) / 2,
        300,
        sg_width,
        500,
        test_pack.clone(),
    );
    // sg.add_widget(&mut test_pack, test_pack.height());
    sg.add_widget(&mut test_pack);

    //SECOND SCROLL GROUP
    let mut test_pack_2 = Pack::new(200, 500, sg_width, 0, "");
    test_pack_2.end();

    for x in 0..20 {
        let mut but = Button::default()
            .with_size(100, 50)
            .with_label(&x.to_string());
        but.set_color(Color::Red);
        test_pack_2.add(&but);
    }

    let mut sg_2 = ScrollGroup::new(
        (win.width() - sg_width+400) / 2,
        300,
        sg_width,
        500,
        test_pack_2.clone(),
    );
    // sg_2.pack.add(&test_pack_2);
    test_pack_2.init_sizes();
    // sg_2.add_widget(&mut test_pack_2, test_pack_2.height());
    sg_2.add_widget(&mut test_pack_2);

    win.end();
    win.show();

    let val = sg.clone();
    win.handle(move |widg, ev| {
        if ev == ButtonEvent::ADD_BUTTON_PUSH.into() {
            dbg!("got add"); 
            // val.add_widget();
            true
        } else if ev == ButtonEvent::SUBTRACT_BUTTON_PUSH.into() {
            dbg!("got subtract");
            true
        } else {
            false
        }
    });

    app::add_timeout(1.0, move||{
        app::redraw();
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg {
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
                AddButton => {
                    dbg!("going to add button");
                }
                RemoveButton => {
                    dbg!("going to remove button");
                }
            }
        }
    }
}
