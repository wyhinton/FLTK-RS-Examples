use fltk::{app, button::*, draw::*, frame::*, group::*, prelude::*};
use fltk::{enums::Color, enums::FrameType, prelude::GroupExt, prelude::WidgetExt};

#[derive(Clone)]
pub struct ScrollBar {
    pub bounds: (i32, i32),
    pub frame: Frame,
    pub value: f32,
    pub height: i32,
    pub size: f32,
}

impl ScrollBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut frame = Frame::new(x, y, w, h, "");
        frame.set_color(Color::Black);
        frame.set_frame(FrameType::FlatBox);
        let sg = ScrollBar {
            frame: frame,
            bounds: (y, y + h),
            value: 0.0,
            height: 0,
            size: 0.50,
        };

        sg
    }
    pub fn set_value(&mut self, val: f32) {
        dbg!(val);
        self.value = val;
        dbg!(self.value);

        let mut frame_cl = self.frame.clone();
        let bar_height = self.size * self.frame.height() as f32;
        frame_cl.draw(move |widg| {
            let should_be_cur_pos = val * widg.height() as f32;
            draw_rect_fill(
                widg.x(),
                widg.y() + should_be_cur_pos as i32,
                widg.width(),
                bar_height as i32,
                Color::White,
            )
        });
        self.frame.redraw();
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size
    }
}

//     pub fn set_parent(&mut self, pack: Pack) {
//         let tc = self.target.clone();
//         self.frame.draw(move |widg| {
//             // self.frame.draw2(move |widg| {
//             dbg!(pack.y(), pack.height());

//             // dbg!(self.target);
//         });
//     }
//     pub fn set_size(&mut self, size: f32) {
//         self.height = (size * self.bounds.1 as f32) as i32;
//     }
//     pub fn scroll_up(&mut self, amount: i32) {
//         dbg!("should scroll up");
//     }

//     pub fn scroll_down(&mut self, amount: i32) {
//         dbg!("should scroll down");
//     }

//     // pub fn set_target(&mut self, pack: Pack) {
//     //     self.target = Some(pack);
//     // }
//     pub fn set_value(&mut self, value: f32) {
//         self.value = value;
//         // dbg!(value);
//         dbg!(self.frame.parent().unwrap().height());
//         match self.target.clone() {
//             Some(target) => {
//                 dbg!(target.height());
//             }
//             None => (),
//         };
//         let mut gx = self.frame.clone();
//         // dbg!(self.target.unwrap().height());
//         dbg!(self.frame.parent().unwrap().height());

//         // let new_y = map_range_clamp((0.0, self.target.unwrap().height()), (0.0, self.frame.parent().unwrap().height(), );
//         // let mapped_height = map_range((0.0, self.target.unwrap().height()), (0.0, self.frame.parent().unwrap().height(), );
//         // dbg!(mapped_height);
//         // gx.draw2(move|widg|{
//         //     widg.resize(widg.x(), widg.y(), 25, mapped_height as i32);
//         // });
//         // self.frame.draw2(move|widg|{
//         //     dbg!(pack.y(), pack.height());

//         //     // dbg!(self.target);
//         // });
//     }
// }
// pub fn map_range_clamp(from_range: (f32, f32), to_range: (f32, f32), mut s: f32) -> f32 {
//     // let in_min = min_max(s, min: f32, max: f32)
//     // let from_range.0 = clamp(from_range.0, )
//     s = s.clamp(from_range.0, from_range.1);
//     // yield "    _value = min(max(value, inMin), inMax) if inMin < inMax else min(max(value, inMax), inMin)"
//     to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
// }

// pub fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
//     to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
// }
