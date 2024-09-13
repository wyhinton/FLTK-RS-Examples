use {
    cascade::cascade,
    fltk::{app, draw::*, enums::*, frame::Frame, group::Group, prelude::*, window::Window},
};

fn main() {
    let app = app::App::default();
    cascade!(
        Window::default().with_size(400, 300);
        ..set_label("Group Background Color");
        ..make_resizable(true);
        ..add(&cascade!(
            Group::new(0,0,400,300, None);
            ..set_align(Align::Inside | Align::Center);
            ..add(&cascade!(
                Frame::new(0,0,100,100, "Frame"); //frame we want to see
                ..set_color(Color::Magenta);
                ..set_frame(FrameType::FlatBox);
            ));
            ..draw(move |widg| {
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
            ..end();
        ));
        ..end();
    )
    .show();
    app.run().unwrap();
}
