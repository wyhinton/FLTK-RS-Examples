use fltk::{app, button::*, draw::*, frame::*, group::*, prelude::*};
use fltk::{enums::Color, enums::FrameType, prelude::GroupExt, prelude::WidgetExt};

#[derive(Clone)]
pub struct ScrollBar {
    pub bounds: (i32, i32),
    pub frame: Frame,
    pub value: f32,
    pub height: i32,
    pub size: f32,
    pub container: Group,
}

impl ScrollBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let group = Group::new(x, y, w, h, None);
        let mut track = Frame::new(x, 0, w, h, "test");
        // let mut track = Frame::new(x, y, w, h, "");
        track.set_frame(FrameType::FlatBox);
        track.set_color(Color::Yellow);

        let mut frame = Frame::new(x, y, w, h, "");
        frame.set_color(Color::Magenta);
        frame.set_frame(FrameType::FlatBox);
        group.end();
        let sg = ScrollBar {
            container: group,
            frame: frame,
            bounds: (y, y + h),
            value: 0.0,
            height: h,
            size: 0.50,
        };
        sg
    }
    pub fn set_value(&mut self, val: f32) {
        self.value = val;
        //TODO: SET DAMAGES
        let bar_height = self.size * self.height as f32;
        let should_be_cur_pos = val * self.height as f32;
        let prev_y = self.container.y();

        self.container.draw(move |widg| {
            widg.draw_children();
            draw_rect_fill(
                widg.x(),
                widg.y(),
                widg.width(),
                widg.height(),
                Color::White,
            );
        });
        self.frame.resize(
            self.frame.x(),
            prev_y + should_be_cur_pos as i32,
            self.frame.width(),
            bar_height as i32,
        );
        self.frame.redraw();
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size
    }
}
