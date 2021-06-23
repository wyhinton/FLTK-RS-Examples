  
use fltk::{app, enums::FrameType, frame::Frame, image::SvgImage, prelude::*, window::Window, button::*};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 500, 500, "Hello from rust");
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}