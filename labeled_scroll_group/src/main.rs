//demonstrates a fuzzy search bar
use fltk::prelude::*;
use fltk::*;
use fltk::{app::*, button::*, frame::*, group::*, image::*, input::*, table::*, window::*};
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
    let mut test_pack = Pack::new(0, 0, sg_width, 100, "");

    test_pack.end();
    let sg = ScrollGroup::new(
        (win.width() - sg_width) / 2,
        300,
        sg_width,
        500,
        test_pack.clone(),
    );

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
