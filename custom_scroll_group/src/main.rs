use fltk::{app, prelude::*, app::*, button::*, enums::*, group::*, window::*};
pub mod scroll_group;
use scroll_group::{ScrollGroup, CustomScrollEvent};
pub mod scroll_bar;
use scroll_bar::ScrollBar;
pub mod events;
pub mod custom_emmiter;
pub mod counter;
use counter::Counter;
use custom_emmiter::CustomEmmiter;
pub mod arc_widget;
use arc_widget::{ArcWidget, ArcGroup};
use std::{sync::{Mutex}};
use event_emitter_rs::EventEmitter;
use serde::{Serialize, Deserialize};

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
    AddButton,
    RemoveButton,
}

fn main() {
    let sg_width = 300;

    let app = App::default();
    let mut win = Window::new(200, 200, 1000, 1000, "Custom Scroll Group");
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
    let mut test_pack = Pack::new(100, 500, sg_width, 0, "");
    test_pack.end();

    for x in 0..20 {
        let mut but = Button::default()
            .with_size(100, 50)
            .with_label(&x.to_string());
        but.set_color(Color::Red);
        test_pack.add(&but);
    }
    test_pack.redraw();

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
    test_pack_2.init_sizes();
    sg_2.add_widget(&mut test_pack_2);

    win.end();
    win.show();

    app::add_timeout(1.0, move||{
        app::redraw();
    });
    app.run().unwrap();
}
