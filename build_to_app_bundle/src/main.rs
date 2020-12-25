//Build application to a MacOS .app and a Window's .msi
//First install https://github.com/burtonageo/cargo-bundle then run cargo bundle --release [target]
//For your OS run bundle --release
//For Windows run bunde --


use fltk::*;
use std::env::{var, set_var};
use std::path::Path;
use fltk::{app::*, frame::*, image::*, window::*};

fn main() {
    let app = App::default();
    let mut win = Window::new(200, 200, 1000, 1000, "My Application");

    let mut img_frame = Frame::default().with_size(200,200).center_of(&win);
    img_frame.set_frame(FrameType::RFlatBox);
    img_frame.set_color(Color::Red);


 
    win.end();
    win.show();


    let (s, r) = app::channel::<bool>();

    while app.wait() {
        if let Some(msg) = r.recv() {

        }
    }
}

