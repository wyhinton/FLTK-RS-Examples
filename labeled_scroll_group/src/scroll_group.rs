use crate::ScrollBar;
use fltk::{
    app, app::MouseWheel, button::*, draw::*, enums::Color, enums::Damage, enums::Event,
    enums::FrameType, frame::*, group::*, prelude::*, widget::*,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ScrollGroup {
    // pub master: Group,
    pub pack: Group,
    pub inner_group: Pack,
    // pub bar: Rc<RefCell<ScrollBar>>,
}
pub fn map_range_clamp(from_range: (f32, f32), to_range: (f32, f32), mut s: f32) -> f32 {
    // let in_min = min_max(s, min: f32, max: f32)
    // let from_range.0 = clamp(from_range.0, )
    s = s.clamp(from_range.0, from_range.1);
    // yield "    _value = min(max(value, inMin), inMax) if inMin < inMax else min(max(value, inMax), inMin)"
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

impl ScrollGroup {
    pub fn new(x: i32, y: i32, w: i32, h: i32, mut inner_group: Pack) -> Self {
        let bar_width = 25;
        let move_size = 5;
        let mut master_container = Group::new(x, y, w, h, "");
        master_container.set_frame(FrameType::FlatBox);
        master_container.set_color(Color::Green);
        master_container.make_resizable(false);
        let mut s_container = Group::new(x, y, w, h, "");
        s_container.set_frame(FrameType::FlatBox);
        s_container.set_color(Color::Black);
        let mut inner_container = Group::new(x, y, w - bar_width, h, "");
        inner_container.set_frame(FrameType::FlatBox);
        inner_container.set_color(Color::Blue);
        //inner contents
        let mut pack = Pack::new(x, y, w - bar_width, 0, "");
        pack.end();
        inner_container.end();
        s_container.end();
        master_container.end();
        //add some buttons
        for x in 0..20 {
            let mut but = Button::default()
                .with_size(100, 50)
                .with_label(&x.to_string());
            but.set_color(Color::Red);
            pack.add(&but);
        }

        let bar_button_x = master_container.x() + master_container.width() - 25;
        let bar_button_y = master_container.y();
        let test_frame = Rc::from(RefCell::from(
            Frame::default()
                .with_size(25, h)
                .with_pos(bar_button_x, bar_button_y),
        ));

        // let track_group = Group::new(bar_button_x, bar_button_y, bar_width, h);

        // let cur_pos = Rc::from(RefCell::from(25));
        let inner_contents_height = Rc::from(RefCell::from(25));
        let inner_contents_pos = Rc::from(RefCell::from(pack.y()));
        let scroll_bar = ScrollBar::new(bar_button_x, bar_button_y, bar_width, h);

        test_frame.borrow_mut().draw({
            let inner_contents_height_clone = inner_contents_height.clone();
            let icp_cl = inner_contents_pos.clone();
            let mut scroll_bar_cl = scroll_bar.clone();
            move |widg| {
                let height_ratio =
                    widg.height() as f32 / inner_contents_height_clone.borrow_mut().clone() as f32;
                let bar_height = (widg.height() as f32 * height_ratio) as i32;

                let dif_height = widg.height() - inner_contents_height_clone.borrow_mut().clone();
                let min_height = widg.y() + dif_height;
                let max_height = widg.y();
                let from_range = (min_height as f32, max_height as f32);

                let float_cur_pos = icp_cl.borrow_mut().clone() as f32;
                dbg!(float_cur_pos);
                let real_pos = map_range_clamp(from_range, (0.0, 1.0), float_cur_pos);

                dbg!(real_pos);
                let cur_percent = 1.0 - real_pos;
                dbg!(cur_percent);
                let should_be_cur_pos = cur_percent * widg.height() as f32;
                dbg!(should_be_cur_pos);

                scroll_bar_cl.set_value(cur_percent);
                scroll_bar_cl.set_size(height_ratio);

                dbg!(height_ratio);
            }
        });
        // test_frame.borrow_mut().set_frame(FrameType::FlatBox);
        // test_frame.borrow_mut().set_color(Color::DarkBlue);
        // test_frame.borrow_mut().set_color(Color::Magenta);

        pack.make_resizable(false);
        inner_container.make_resizable(false);

        let pack_cl2 = pack.clone();
        master_container.set_clip_children(true);

        let mut sss = s_container.clone();
        inner_container.handle({
            let mut scroll_bar_clone = scroll_bar.clone();
            move |s, ev| match ev {
                Event::MouseWheel => {
                    // test_frame_cl.redraw();
                    test_frame.borrow_mut().redraw();

                    let dy = app::event_dy();
                    match dy {
                        MouseWheel::Up => {
                            // *cur_pos_cl.borrow_mut() += move_size;
                            pack.resize(sss.x(), pack.y() + move_size, pack.width(), pack.height());
                            pack.redraw();
                            *inner_contents_height.borrow_mut() = pack.height();
                            *inner_contents_pos.borrow_mut() = pack.y();
                            s.set_damage_type(Damage::Scroll);
                            sss.set_damage_type(Damage::Scroll);
                        }
                        MouseWheel::Down => {
                            // *cur_pos_cl.borrow_mut() -= move_size;
                            pack.resize(sss.x(), pack.y() - move_size, pack.width(), pack.height());
                            pack.redraw();
                            *inner_contents_height.borrow_mut() = pack.height();
                            *inner_contents_pos.borrow_mut() = pack.y();
                            s.set_damage_type(Damage::Scroll);
                            sss.set_damage_type(Damage::Scroll);
                        }
                        _ => (),
                    }
                    true
                }

                _ => false,
            }
        });

        let sg = ScrollGroup {
            // master: win,
            pack: inner_container,
            inner_group: pack_cl2,
            // bar: bar,
            // pos: 0,
        };

        sg
    }
}

impl std::ops::Deref for ScrollGroup {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl std::ops::DerefMut for ScrollGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}

struct Label {
    pub frame: Frame,
}
impl Label {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut frame = Frame::new(x, y, w, 25, "Title");
        frame.set_frame(FrameType::FlatBox);
        frame.set_color(Color::Green);
        return Label { frame: frame };
    }
}
