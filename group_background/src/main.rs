use fltk::{app, prelude::*, frame::Frame, window::Window, enums::*, group::Group, draw::*};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300).with_label("Group Background Color");
    wind.make_resizable(true);
    let mut group = Group::new(0,0,400,300, None);
    group.set_align(Align::Inside | Align::Center);
    //frame we want to see
    let mut frame = Frame::new(0,0,100,100, "Frame");
    frame.set_color(Color::Magenta);
    frame.set_frame(FrameType::FlatBox);
    group.end();
    group.draw(move |widg| {
        //background fill
        draw_rect_fill(
            widg.x(),
            widg.y(),
            widg.width(),
            widg.height(),
            Color::Red,
        );
        //draw the children after drawing the background fill color
        widg.draw_children();
    });
    wind.end();
    wind.show();
    app.run().unwrap();
}