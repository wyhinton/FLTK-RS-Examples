use fltk::{prelude::*, *, *, group::*, enums::*, button::*};
use std::ops::{Deref, DerefMut};
use state::Storage;

pub struct MyButton {
    grp: group::Group,
}

impl MyButton {
    pub fn new(w: i32, h: i32) -> MyButton {
        let mut grp = Group::new(0, 0, w, h, None);
        grp.set_frame(FrameType::RFlatBox);
        grp.set_color(Color::from_u32(0x01579b));
        grp.set_align(Align::Center);
        let mut btn = Button::new(grp.x() + 420, grp.y() + 35, 15, 15, "X");
        btn.set_frame(FrameType::OFlatFrame);
        btn.set_color(Color::from_u32(0xf49da9));
        btn.set_callback(move |b| b.parent().unwrap().hide());
        grp.end();
        grp.handle(|g, ev| match ev {
            Event::Push => {
                g.do_callback();
                true
            }
            _ => false,
        });
        MyButton { grp }
    }
}

impl Deref for MyButton {
    type Target = group::Group;
    fn deref(&self) -> &Self::Target {
        &self.grp
    }
}
impl DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grp
    }
}
