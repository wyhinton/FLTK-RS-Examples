use crate::{events::ScrollBarEvent, ScrollBar};
use fltk::{
    app, app::MouseWheel, button::*, draw::*, enums::Color, enums::Damage, enums::Event,
    enums::FrameType, frame::*, group::*, prelude::*, widget::*,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ScrollGroupEvent;

impl ScrollGroupEvent {
    const WIDGET_ADDED: i32 = 43; // values below 30 are reserved
    const WIDGET_REMOVED: i32 = 44;
}

#[derive(Clone)]
pub struct ScrollGroup {
    pub pack: Group,
    pub inner_group: Pack,
}
pub fn map_range_clamp(from_range: (f32, f32), to_range: (f32, f32), mut s: f32) -> f32 {
    s = s.clamp(from_range.0, from_range.1);
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

impl ScrollGroup {
    pub fn new(x: i32, y: i32, w: i32, h: i32, mut inner_group: Pack) -> Self {
        let bar_width = 25;
        let mut inner_container = Group::new(x, y, w, h, "");
        inner_container.set_frame(FrameType::FlatBox);
        inner_container.set_color(Color::Blue);

        let bar_button_x = x + w - bar_width;
        let bar_button_y = inner_container.y();
        let scroll_bar_ref = Rc::from(RefCell::from(ScrollBar::new(
            bar_button_x,
            bar_button_y,
            bar_width,
            h,
        )));
        //inner contents
        let mut pack = inner_group.clone();
        scroll_bar_ref
            .borrow_mut()
            .set_size((h as f32 / pack.height() as f32) as f32);
        pack.resize(inner_container.x(), inner_container.y(), w - bar_width, 0);
        pack.end();

        inner_container.end();
        inner_container.set_clip_children(true);

        pack.make_resizable(false);
        inner_container.make_resizable(false);

        let pack_cl2 = pack.clone();
        let scroll_bar_ref_cl = scroll_bar_ref.clone();

        inner_container.handle({
            move |widg, ev| {
                if ev == ScrollBarEvent::RUNNER_DRAGGED.into() {
                    // dbg!("got at parent");
                    let dif_height = widg.height() - pack.height();
                    let min_height = (widg.y() + dif_height) as f32;
                    let max_height = widg.y() as f32;
                    let new_val = map_range_clamp(
                        (0.0, 1.0),
                        (min_height, max_height),
                        scroll_bar_ref_cl.borrow().value(),
                    );
                    pack.resize(pack.x(), new_val as i32, pack.width(), pack.height());
                    widg.redraw();
                    true
                } else if ev == ScrollBarEvent::TRACK_CLICKED.into() {
                    dbg!("got track clicked");
                    let dif_height = widg.height() - pack.height();
                    let min_height = (widg.y() + dif_height) as f32;
                    let max_height = widg.y() as f32;
                    dbg!(scroll_bar_ref_cl.borrow().value());
                    let new_val = map_range_clamp(
                        (0.0, 1.0),
                        (min_height, max_height),
                        scroll_bar_ref_cl.borrow().value(),
                    );
                    pack.resize(pack.x(), new_val as i32, pack.width(), pack.height());
                    widg.redraw();
                    true
                } else {
                    match ev {
                        Event::Resize => {
                            dbg!("got scroll group resize");
                            widg.resize(x, y, w, h);
                            widg.redraw();
                            true
                        }
                        Event::MouseWheel => {
                            let dy = app::event_dy();
                            let dif_height = widg.height() - pack.height();
                            let min_height = (widg.y() + dif_height) as f32;
                            let max_height = widg.y() as f32;
                            dbg!("got mousewehel");
                            dbg!(*scroll_bar_ref_cl.borrow_mut().runner.value.borrow_mut());
                            let new_val = map_range_clamp(
                                (0.0, 1.0),
                                (min_height, max_height),
                                *scroll_bar_ref_cl.borrow_mut().runner.value.borrow_mut(), // scroll_bar_ref_cl.borrow_mut().value(),
                            );

                            match dy {
                                MouseWheel::Up => {
                                    scroll_bar_ref_cl.borrow_mut().increase_value();
                                    pack.resize(
                                        pack.x(),
                                        new_val as i32,
                                        pack.width(),
                                        pack.height(),
                                    );
                                    pack.redraw();
                                    widg.set_damage_type(Damage::Scroll);
                                }
                                MouseWheel::Down => {
                                    scroll_bar_ref_cl.borrow_mut().decrease_value();
                                    pack.resize(
                                        pack.x(),
                                        new_val as i32,
                                        pack.width(),
                                        pack.height(),
                                    );
                                    pack.redraw();
                                    widg.set_damage_type(Damage::Scroll);
                                }
                                _ => (),
                            }
                            true
                        }

                        _ => false,
                    }
                }
            }
        });

        let sg = ScrollGroup {
            pack: inner_container,
            inner_group: pack_cl2,
        };

        sg
    }
    pub fn add_widget(&mut self, to_add: &impl WidgetExt) {
        self.inner_group.add(to_add);
        let _ = app::handle_main(ScrollGroupEvent::WIDGET_ADDED);
    }
    pub fn remove_widget(&mut self, to_remove: &impl WidgetExt) {
        // self.grp.remove(wid);
        // app::handle_main(MyEvent::REMOVE_WIDGET);
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
