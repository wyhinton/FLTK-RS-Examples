//a custom TableButton widget for our table

use fltk::{widget::Widget, enums::*, prelude::*, draw::*};
use std::ops::{Deref, DerefMut};

pub struct TableButton{
    pub wid: Widget,
}

impl TableButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> TableButton {
        let mut tb = TableButton{
            wid: Widget::new(x,y,w,h, None)
        };
        tb.draw();
        tb.handle();
        tb 
    }
    fn draw(&mut self) {
        let padding = 10;
        self.wid.draw(move |b| {
            draw_box(
                FrameType::GtkDownBox,
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Color::from_u32(0xFFC300),
            );
            set_draw_color(Color::Black);
            draw_rect(
                b.x(),
                b.y(),
                10, 
                10, 
            );
            let tt = &b.label()[..];
            let (tx, ty) = fltk::draw::measure(&tt, false);
            if tx > b.width() - padding {
                let fin = vec![&b.label()[..6], "…"].join("");
                draw_text(&fin[..], b.x() + padding , b.y() + 50);
            } else {
                draw_text(&b.label(), b.x() + padding , b.y() + 50);
            }
        });
    }
    fn handle(&mut self) {
        let mut wid = self.wid.clone();
        self.wid.handle(move |widg, ev| match ev {
            Event::Push => {
                wid.do_callback();
                true
            }
            _ => false,
        });
    }
}
impl Deref for TableButton {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for TableButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
