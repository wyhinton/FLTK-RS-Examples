//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*,frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
use rand::{distributions::Alphanumeric, Rng};
pub mod scroll_group;
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
    let mut win = Window::new(200, 200, 1000, 500, "Scroll Group");
    win.make_resizable(true);
    app::background(67,67,67);
    app::background2(100,100,100);
    let bg =Frame::new(0,0,1000,500,"");
    bg.set_frame(FrameType::FlatBox);
    bg.set_color(Color::Blue);
    let mut scroll_group = ScrollGroup::new(0,300,200,200);

    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
            }
        }
    }
}

