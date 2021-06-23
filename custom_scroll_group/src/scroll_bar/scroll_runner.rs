use fltk::{app, draw::*, enums::*, frame::*, group::*, prelude::*};
use fltk::{enums::Color, enums::FrameType, prelude::GroupExt, prelude::WidgetExt};
use crate::{CustomEmmiter, Counter, ArcWidget, CustomScrollEvent, WidgDimensions};

#[derive(Clone)]
pub struct Runner {
    pub frame: Frame,
    pub arc_val: Counter<f32>, 
    pub arc_pos: (Counter<i32>, Counter<i32>),
}

pub struct CustomRunnerEvent{}

impl CustomRunnerEvent{
    pub const SET_RUNNER_POS: &'static str = "SET_RUNNER_POS";
    pub const INCREMENT_RUNNER_POS: &'static str = "INCREMENT_RUNNER_POS";
    pub const SET_RUNNER_VALUE: &'static str = "SET_RUNNER_VALUE";
    pub const UPDATE_SCROLL_GROUP: &'static str = "UPDATE_SCROLL_GROUP";
}

impl Runner {
    pub fn new(x: i32, y: i32, w: i32, h: i32, arc_val: Counter::<f32>, emmiter: CustomEmmiter) -> Self {
        let mut frame = Frame::new(x, y, w, h, None);
        frame.set_color(Color::Magenta);
        frame.set_frame(FrameType::FlatBox);
        // frame.make_resizable(false);
        let arc_frame = ArcWidget::<Frame>::new(frame.clone());

        //ARCARCARCARCARCARCARCARC
        //offset
        let arc_offset = (Counter::<i32>::new(0), Counter::<i32>::new(0));
        //runner value
        let arc_val_cl_2 = arc_val.clone();
        let arc_size = Counter::<i32>::new(100);

        //handle addition/subtraction of children from the scroll group
        let arc_frame_cl = arc_frame.clone();
        let arc_size_cl = arc_size.clone();

        let emmiter_cl_4 = emmiter.clone();
        emmiter.on(CustomScrollEvent::CHILD_RESIZE, move|value:  WidgDimensions|{
            let relative =  (arc_frame_cl.parent().unwrap().height() as f32 / value.height as f32);
            let new_runner_height = relative*arc_frame_cl.parent().unwrap().height() as f32;
            dbg!(arc_frame_cl.height()-new_runner_height as i32);
            let height_difference = arc_frame_cl.height()-new_runner_height as i32;
            let new_y_pos = arc_frame_cl.y()-height_difference;
            dbg!(new_y_pos);
            arc_size_cl.set(new_runner_height as i32);
            emmiter_cl_4.emit(CustomRunnerEvent::SET_RUNNER_POS, (arc_frame_cl.x(), new_y_pos));
            arc_frame_cl.set_height(new_runner_height as i32);
            // arc_frame_cl.redraw();
            // arc_frame_cl.parent().unwrap().redraw();
        });

        let emmiter_cl = emmiter.clone();

        //handle change to runner position
        let arc_frame_cl_2 = arc_frame.clone();
        let arc_size_cl_2 = arc_size.clone();
        emmiter.on(CustomRunnerEvent::SET_RUNNER_POS, move|value: (i32, i32)|{
            dbg!("got set runner pos");
            let p_y = arc_frame_cl_2.parent().unwrap().y();
            let p_h = arc_frame_cl_2.parent().unwrap().height()-arc_size_cl_2.get();
            // arc_frame_cl_2.set_pos(arc_frame_cl_2.x(), value.1.clamp(p_y,p_y+p_h));
            dbg!(p_y, p_h, arc_size_cl_2.get());
            arc_frame_cl_2.set_pos(arc_frame_cl_2.x(), value.1.clamp(p_y,p_y+p_h));
            arc_frame_cl_2.parent().unwrap().set_damage_type(Damage::User1);
            arc_frame_cl_2.parent().unwrap().parent().unwrap().set_damage_type(Damage::User1);
            arc_frame_cl_2.set_damage_type(Damage::User1);
            dbg!(value);
            let tva = value.1 - p_y;
            let new_val = ((tva as f32/(p_h) as f32)).clamp(0.0,1.0);
            arc_val_cl_2.set(new_val);
            emmiter_cl.emit(CustomRunnerEvent::UPDATE_SCROLL_GROUP, new_val);
        });

        //handle mouse scrolling
        let arc_frame_cl_2 = arc_frame.clone();
        let emmiter_cl_2 = emmiter.clone();
        emmiter.on(CustomRunnerEvent::INCREMENT_RUNNER_POS, move|value: i32|{
            let new_y = arc_frame_cl_2.y() + value; 
            let new_x = arc_frame_cl_2.x();
            emmiter_cl_2.emit(CustomRunnerEvent::SET_RUNNER_POS, (new_x, new_y));

        });

        emmiter.on(CustomScrollEvent::CHILD_RESIZE, move|value: i32|{
           dbg!("got child added");
        });

        let emmiter_cl = emmiter.clone();
        let count = Counter::<i32>::new(0);
        frame.handle(move |widg, ev| match ev {
            Event::Drag => {
                count.increment(1);
                dbg!("got drag");
                if (count.get() % 3) == 0{
                    emmiter_cl.emit(CustomRunnerEvent::SET_RUNNER_POS, (app::event_x(), app::event_y()));
                }
                true
            }
            // Event::Resize=>{
            //     dbg!("got frame resize");
            //     dbg!(widg.width());
            //     true
            // }
            _ => false,
        });
        Runner {
            frame: frame,
            arc_pos: arc_offset,
            arc_val: arc_val,
        }
    }
}


pub fn map_range_clamp(from_range: (f32, f32), to_range: (f32, f32), mut s: f32) -> f32 {
    s = s.clamp(from_range.0, from_range.1);
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
