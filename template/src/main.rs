//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
use rand::{distributions::Alphanumeric, Rng};


#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    

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

