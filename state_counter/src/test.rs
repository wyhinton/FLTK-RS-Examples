use fltk::{app, button::Button, group::*, enums::*, prelude::*, window::Window};


fn main(){
    let app = app::App::default();
    // let mut wind = Window::default().with_size(500, 200).with_label("Counter");
    let mut wind = Window::default().with_size(500, 200).with_label("Counter");

    let dummy_button = Button::new(100,100,100,100, "dummy");
    let dummy_button = Button::new(200,100,100,100, "dummy");
    wind.end();
    wind.show();
    wind.make_resizable(true);
    app.run().unwrap();
}
