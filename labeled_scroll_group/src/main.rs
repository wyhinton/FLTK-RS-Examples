use fltk::prelude::*;
use fltk::*;
use fltk::{app::*, button::*, enums::*, group::*, window::*};
pub mod scroll_group;
use fltk::{prelude::GroupExt, prelude::WidgetExt};
use scroll_group::ScrollGroup;
pub mod scroll_bar;
use scroll_bar::ScrollBar;
pub mod events;

pub struct ButtonEvent;

impl ButtonEvent {
    const ADD_BUTTON_PUSH: i32 = 43; // values below 30 are reserved
    const SUBTRACT_BUTTON_PUSH: i32 = 44;
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

    let mut add_button = Button::new(0, 0, 150, 50, "Add Button");
    add_button.set_callback(move |_| {
        let _ = app::handle_main(ButtonEvent::ADD_BUTTON_PUSH);
    });
    let mut subtract_button = Button::new(0, 0, 150, 50, "Subtract Button");
    subtract_button.set_callback(move |_| {
        let _ = app::handle_main(ButtonEvent::SUBTRACT_BUTTON_PUSH);
    });

    button_container.end();
    button_container.set_type(PackType::Horizontal);

    let mut test_pack = Pack::new(100, 500, sg_width, 1000, "");
    test_pack.end();

    for x in 0..20 {
        let mut but = Button::default()
            .with_size(100, 50)
            .with_label(&x.to_string());
        but.set_color(Color::Red);
        test_pack.add(&but);
    }

    let mut sg = ScrollGroup::new(
        (win.width() - sg_width) / 2,
        300,
        sg_width,
        500,
        test_pack.clone(),
    );
    sg.pack.add(&test_pack);

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
