use fltk::{app, enums::*, group::*, prelude::*};
use fltk::{enums::Color, enums::FrameType, prelude::{WidgetExt}};
use crate::{CustomEmmiter, Counter, ArcGroup};
pub mod scroll_runner;
use scroll_runner::CustomRunnerEvent;
use scroll_runner::Runner;


struct CustomScrollRunnerEvent{}

impl CustomScrollRunnerEvent{
    pub const TRACK_CLICKED: &'static str = "TRACK_CLICKED";
}

#[derive(Clone)]
pub struct ScrollBar {
    pub runner: Runner,
    pub bounds: (f32, f32),
    pub container: ArcGroup<Group>,
}

impl ScrollBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32, emmiter: CustomEmmiter) -> Self {

        let emmiter_cl = emmiter.clone();
        emmiter.on(CustomScrollRunnerEvent::TRACK_CLICKED, move|value: (i32, i32)|{
           dbg!(value);
           emmiter_cl.emit(CustomRunnerEvent::SET_RUNNER_POS, value);
        });

        let group = ArcGroup::<Group>::new(Group::new(x, y, w, h, None));
        group.set_frame(FrameType::FlatBox);
        group.set_color(Color::Dark2);

        group.draw(move|widg|{
            let x = widg.parent().unwrap().x()+ widg.parent().unwrap().width()-25;
            widg.set_pos(x,  widg.parent().unwrap().y());
        });
        
        let arc_val = Counter::<f32>::new(1.0);
        let runner = Runner::new(x, y, w, h,arc_val.clone(),  emmiter.clone());
        group.end();

        let runner_frame_cl = runner.frame.clone();
        let emmiter_cl = emmiter.clone();
        group.handle(move |_, ev| match ev {
            Event::Push => {
                if !app::event_inside_widget(&runner_frame_cl) {
                    emmiter_cl.emit(CustomScrollRunnerEvent::TRACK_CLICKED, app::event_coords());
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
        };

        sg
    }
    pub fn set_value(&mut self, val: f32) {
        self.runner.arc_val.set(val);
    }
    pub fn increase_value(&mut self) {
        let old_val_arc = self.runner.arc_val.get();
        self.runner.arc_val.set((old_val_arc + 0.01).clamp(self.bounds.0, self.bounds.1)); 
        self.runner.frame.do_callback();
    }
    pub fn decrease_value(&mut self) {
        let old_val_arc = self.runner.arc_val.get();
        self.runner.arc_val.set((old_val_arc - 0.01).clamp(self.bounds.0, self.bounds.1)); 
        self.runner.frame.do_callback();
    }
    pub fn value(&self) -> f32 {
        self.runner.arc_val.get()
    }
}


pub fn map_range_clamp(from_range: (f32, f32), to_range: (f32, f32), mut s: f32) -> f32 {
    s = s.clamp(from_range.0, from_range.1);
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
