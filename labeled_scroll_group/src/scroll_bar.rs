use crate::events::ScrollBarEvent;
use fltk::{app, draw::*, enums::*, frame::*, group::*, prelude::*, widget::Widget};
use fltk::{enums::Color, enums::FrameType, prelude::GroupExt, prelude::WidgetExt};
use std::cell::RefCell;
use std::rc::Rc;

// pub struct ScrollBarEvent;

// impl ScrollBarEvent {
//     pub const RUNNER_DRAGGED: i32 = 40; // values below 30 are reserved
//     pub const TRACK_CLICKED: i32 = 41;
// }

#[derive(Clone)]
pub struct ScrollBar {
    pub runner: Runner,
    pub bounds: (f32, f32),
    pub height: Rc<RefCell<i32>>,
    pub size: Rc<RefCell<f32>>,
    pub container: Group,
}
#[derive(Clone)]
pub struct Runner {
    pub frame: Frame,
    pub value: Rc<RefCell<f32>>,
    pub pos: Rc<RefCell<(i32, i32)>>,
    pub size: Rc<RefCell<f32>>,
}

impl Runner {
    pub fn new(x: i32, y: i32, w: i32, h: i32, val: Rc<RefCell<f32>>) -> Self {
        let mut frame = Frame::new(x, y, w, h, None);
        frame.set_color(Color::Magenta);
        frame.set_frame(FrameType::FlatBox);

        let offset = Rc::from(RefCell::from((x, y)));
        let size = Rc::from(RefCell::from(0.0));
        let val_cl = val.clone();

        let size_cl = size.clone();
        frame.set_callback(move |widg| {
            let p_y = widg.parent().unwrap().y();
            let p_h = widg.parent().unwrap().height();
            let new_y = map_range_clamp(
                (0.0, 1.0),
                (
                    p_y as f32,
                    ((p_y + p_h) as f32) - *size_cl.borrow() * h as f32,
                ),
                1.0 - *val_cl.borrow_mut(),
            );
            widg.resize(
                widg.x(),
                new_y as i32,
                widg.width(),
                (*size_cl.borrow() * h as f32) as i32,
            );
        });

        let off_cl = offset.clone();
        let val_cl_2 = val.clone();
        frame.handle(move |widg, ev| match ev {
            Event::Push => {
                *off_cl.borrow_mut() = (app::event_x(), app::event_y());
                true
            }
            Event::Drag => {
                let delta_x = off_cl.borrow_mut().0 - app::event_x();
                let delta_y = off_cl.borrow_mut().1 - app::event_y();
                let parent_size = widg.parent().unwrap().height();
                let test_inc = delta_y as f32 / parent_size as f32;
                *val_cl_2.borrow_mut() += test_inc;
                widg.do_callback();
                *off_cl.borrow_mut() = (app::event_x(), app::event_y());
                (*off_cl.borrow_mut()).1 = app::event_y();
                let _ = app::handle_main(ScrollBarEvent::RUNNER_DRAGGED);
                true
            }
            _ => false,
        });
        Runner {
            frame: frame,
            pos: offset,
            value: val,
            size: size,
        }
    }
}

impl ScrollBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut group = Group::new(x, y, w, h, None);
        let test_val = Rc::from(RefCell::from(1.0));
        let runner = Runner::new(x, y, w, h, test_val.clone());
        group.end();
        let size = Rc::from(RefCell::from(0.5));
        let height = Rc::from(RefCell::from(h));

        let mut frame = unsafe { Widget::from_widget_ptr(runner.frame.as_widget_ptr()) };
        group.draw(move |widg| {
            draw_rect_fill(
                widg.x(),
                widg.y(),
                widg.width(),
                widg.height(),
                Color::White,
            );
            widg.draw_child(&mut frame);
        });
        let mut test_val_cl = test_val.clone();
        let mut runner_frame_cl = runner.frame.clone();
        group.handle(move |widg, ev| match ev {
            Event::Push => {
                if !app::event_inside_widget(&runner_frame_cl) {
                    let _ = app::handle_main(ScrollBarEvent::TRACK_CLICKED);
                    let distance_from_top = app::event_y() - widg.y();
                    let new_val = 1.0 - (distance_from_top as f32 / widg.height() as f32);
                    *test_val_cl.borrow_mut() = new_val;
                    runner_frame_cl.do_callback();
                    true
                } else {
                    false
                }
            }
            _ => false,
        });
        let sg = ScrollBar {
            runner: runner,
            container: group,
            bounds: (0.0, 1.0),
            height: height,
            size: size,
        };

        sg
    }
    pub fn set_value(&mut self, val: f32) {
        *self.runner.value.borrow_mut() = val.to_owned();
    }
    pub fn increase_value(&mut self) {
        let old_val = *self.runner.value.to_owned().borrow();
        *self.runner.value.borrow_mut() = (old_val + 0.01).clamp(self.bounds.0, self.bounds.1);
        dbg!(*self.runner.value.borrow_mut());
        self.runner.frame.do_callback();
    }
    pub fn decrease_value(&mut self) {
        let old_val = *self.runner.value.to_owned().borrow();
        *self.runner.value.borrow_mut() = (old_val - 0.01).clamp(self.bounds.0, self.bounds.1);
        self.runner.frame.do_callback();
    }
    pub fn set_size(&mut self, size: f32) {
        dbg!(size);
        *self.runner.size.borrow_mut() = size;
    }
    pub fn value(&self) -> f32 {
        *self.runner.value.borrow()
    }
}

pub fn map_range_clamp(from_range: (f32, f32), to_range: (f32, f32), mut s: f32) -> f32 {
    s = s.clamp(from_range.0, from_range.1);
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
