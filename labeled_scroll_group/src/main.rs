//demonstrates a fuzzy search bar
use fltk::prelude::*;
use fltk::*;
use fltk::{app::*, button::*, enums::*, group::*, window::*};
pub mod scroll_group;
use fltk::{prelude::GroupExt, prelude::WidgetExt};
use scroll_group::ScrollGroup;
pub mod scroll_bar;
use scroll_bar::ScrollBar;

#[derive(Debug, Clone)]
pub enum Message {
    Test,
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    win.make_resizable(true);
    let sg_width = 300;
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

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg {
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
            }
        }
    }
}
