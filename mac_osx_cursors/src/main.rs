use fltk::{app, app::*, window::*, prelude::*, window::Window, button::*, group::*};
use std::fs;
use image::*; 


#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

fn main() {
    let app = App::default();
    let (s, r) = app::channel::<Message>();

    let mut win = Window::default()
    .with_label("MacOsx Cursor ")
    .with_size(1200, 300)
    .center_screen();
    let cursor_paths = fs::read_dir("assets/cursors/").unwrap();

    let mut button_group = Pack::new(75,150,300, 24,"");
    button_group.set_type(PackType::Horizontal);

    for x in cursor_paths.into_iter(){
        let mut but = Button::new(0,0,25,25,"");
        let sc = s.clone();
        let win_cl = win.clone();
        let img = image::open(x.unwrap().path().display().to_string()).unwrap();
        let bytes = img.clone().into_rgba8().into_vec();
        let rgb = fltk::image::RgbImage::new(&bytes, img.clone().width() as i32, img.clone().height() as i32, fltk::enums::ColorDepth::Rgba8).unwrap();
        
        let rgb_cl = rgb.clone();
        but.set_callback(move |_|{
            win_cl.clone().set_cursor_image(rgb.clone(), 0,0);
        });
        but.set_image(Some(rgb_cl))
    }
    button_group.end();
    win.end();
    win.show();

    while app.wait() {
        // dbg!(app::event_coords());
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

