use crate::{events::ScrollBarEvent, WidgDimensions, ScrollBar, CustomEmmiter};
use fltk::{
    app, app::MouseWheel, button::*, draw::*, enums::Color, enums::Damage, enums::Event,
    enums::FrameType, frame::*, group::*, prelude::*, widget::*,
};
use std::cell::RefCell;
use std::{rc::Rc};
use std::{sync::{Arc, Mutex}};

use crate::{scroll_bar::scroll_runner::CustomRunnerEvent, GLOBAL_EVENT_EMMITER, ButtonEvent, GlobalEvent, ArcGroup};
// use scroll_bar::CustomRunnerEvent; 
pub struct ScrollGroupEvent;



impl ScrollGroupEvent {
    pub const WIDGET_ADDED: i32 = 43; // values below 30 are reserved
    pub const WIDGET_REMOVED: i32 = 44;
}

pub struct CustomScrollEvent;

impl CustomScrollEvent {
    pub const CHILD_RESIZE: &'static str = "WIDGET_ADDED";
    pub const CONTAINER_RESIZE: &'static str = "CONTAINER_RESIZE";
}
#[derive(Clone)]
pub struct ScrollGroup {
    pub pack: Group,
    pub inner_group: Pack,
    pub emmiter: CustomEmmiter, 
    
}
pub fn map_range_clamp(from_range: (f32, f32), to_range: (f32, f32), mut s: f32) -> f32 {
    s = s.clamp(from_range.0, from_range.1);
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
static BAR_WIDTH: i32 = 25;

impl ScrollGroup {
    pub fn new(x: i32, y: i32, w: i32, h: i32, mut inner_group: Pack) -> Self {
        //sender receiver for scroll group
        let emmiter = CustomEmmiter::new();

        //group for holidng scrollable elements and the scroll bar
        let mut master_container = Group::new(x, y, w, h, "");
        master_container.set_frame(FrameType::FlatBox);
        master_container.set_color(Color::Blue);
        let master_container_arc = ArcGroup::<Group>::new(master_container.clone());

        inner_group.draw(move|widg|{
            let x = widg.parent().unwrap().x();
            let y = widg.parent().unwrap().y();
            let width = widg.parent().unwrap().width();
            let height = widg.parent().unwrap().height();
            widg.resize(widg.x(), widg.y(), width-25, widg.height());
        });
        //create arc for scroll contents
        let arc_pack = ArcGroup::<Pack>::new(inner_group.clone());

        //position scroll bar
        let bar_button_x = x + w - BAR_WIDTH;
        let bar_button_y = master_container.y();

        //create scroll bar
        let sb = ScrollBar::new(
            bar_button_x,
            bar_button_y,
            BAR_WIDTH,
            h,
            emmiter.clone());

        let sb_val = sb.runner.arc_val;
        let sb_val_cl = sb_val.clone();

        //master
        let master_container_arc_cl = master_container_arc.clone();

        //sub
        let arc_pack_cl_3 = arc_pack.clone(); 

        //move inner contents in accordance with updates to runner's value
        let igg = inner_group.clone();
        emmiter.on(CustomRunnerEvent::UPDATE_SCROLL_GROUP, move|value:  f32|{
            // dbg!("gout update");
            let end_of_container_pos = master_container_arc_cl.y()+master_container_arc_cl.height();
            let min_height = end_of_container_pos as f32 - arc_pack_cl_3.height() as f32;
            let max_height =  master_container_arc_cl.y() as f32;
            let new_val = map_range_clamp(
                (0.0, 1.0),
                (min_height, max_height),
                1.0-value,
            );
            arc_pack_cl_3.widg().set_pos(arc_pack_cl_3.x(), new_val as i32);
            
            arc_pack_cl_3.widg().redraw();
            let tt = arc_pack_cl_3.clone();
        });
              
        //handle add button event
        let arc_pack_cl = arc_pack.clone();
        let emmiter_cl = emmiter.clone();
        GLOBAL_EVENT_EMMITER.lock().unwrap().on(GlobalEvent::ADD_BUTTON, move|value:  i32|{
            let mut button = Button::default().with_size(100,50);
            let label = arc_pack_cl.children().to_string();
            button.set_label(&label);
            arc_pack_cl.widg().add(&button);
            let new_height = arc_pack_cl.height()+button.height();
            arc_pack_cl.widg().set_damage(true);
            arc_pack_cl.widg().init_sizes();
            let width = arc_pack_cl.widg().width();
            let new_pack_dims = WidgDimensions::new(width, new_height);
            emmiter_cl.emit(CustomScrollEvent::CHILD_RESIZE, new_pack_dims);
            
        });

        //handle remove button event
        let arc_pack_cl_2 = arc_pack.clone();
        let emmiter_cl_2 = emmiter.clone();
        GLOBAL_EVENT_EMMITER.lock().unwrap().on(GlobalEvent::REMOVE_BUTTON, move|value:  i32|{
            let child = arc_pack_cl_2.widg().child(arc_pack_cl_2.children()-1).unwrap();
            let test = unsafe { Widget::from_widget_ptr(child.as_widget_ptr()) };
            let to_remove_height = child.height();
            let width = arc_pack_cl_2.width();
            let new_height = arc_pack_cl_2.height()-to_remove_height;
            WidgetBase::delete(test);
            let new_pack_dims = WidgDimensions::new(width, new_height);
            emmiter_cl_2.emit(CustomScrollEvent::CHILD_RESIZE, new_pack_dims);
            // arc_pack_cl_2.redraw();
        });

        //inner contents
        let mut pack = inner_group.clone();
        pack.resize(master_container.x(), master_container.y(), w - BAR_WIDTH, 0);

        pack.end();

        master_container.end();
        master_container.set_clip_children(true);

        pack.make_resizable(false);
        master_container.make_resizable(false);
        
        let pack_cl2 = pack.clone();
        // let scroll_bar_ref_cl = scroll_bar_ref.clone();
        let emmiter_cl = emmiter.clone();
        master_container.handle({
            move |widg, ev| {
                    match ev {
                        Event::Resize => {
                            dbg!("got reszie");
                            widg.set_damage(false);
                            emmiter_cl.emit(CustomScrollEvent::CONTAINER_RESIZE, (widg.width(), widg.height()));
                            // dbg!(widg.parent().unwrap().parent().unwrap().width());
                            true
                        }
                        Event::MouseWheel => {
                            let dy = app::event_dy();
                            match dy {
                                MouseWheel::Up => {
                                    emmiter_cl.emit(CustomRunnerEvent::INCREMENT_RUNNER_POS, 10);
                                    widg.set_damage_type(Damage::Scroll);
                                }
                                MouseWheel::Down => {
                                    emmiter_cl.emit(CustomRunnerEvent::INCREMENT_RUNNER_POS, -10);
                                    widg.set_damage_type(Damage::Scroll);
                                }
                                _ => (),
                            }
                            true
                        }

                        _ => false,
                    }
                }
        });

        let sg = ScrollGroup {
            pack: master_container,
            inner_group: pack_cl2,
            emmiter: emmiter,
        };

        sg
    }
    pub fn add_widget(&mut self, to_add: &mut impl GroupExt) {
    // pub fn add_widget(&mut self, to_add: &mut impl GroupExt, height: i32) {
        // dbg!(to_add.children());
        to_add.init_sizes();
        dbg!(to_add.height());
        dbg!(to_add.children());
        let new_height = 0;
        let mut new_width = to_add.width();
        for x in 0..to_add.children() {
            // dbg!(to_add.child(x).unwrap().height());
            new_width += to_add.child(x).unwrap().height();
            // dbg!(to_add.width());

        }
        // dbg!(height);

        self.pack.add(to_add);
        self.pack.redraw();
        // to_add.redraw();
        dbg!(to_add.height());
        let height = to_add.height();
        // let new_height = self.pack.height() as f32/to_add.height() as f32;
        // self.emmiter.emit(CustomScrollEvent::CHILD_RESIZE, new_height);
        self.emmiter.emit(CustomScrollEvent::CHILD_RESIZE, WidgDimensions::new(new_height, new_width));
        // let _ = app::handle_main(ScrollGroupEvent::WIDGET_ADDED);
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
