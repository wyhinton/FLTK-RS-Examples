use fltk::frame::*;
use fltk::{app, image::*, group::*, input::*, draw::*};


pub fn inside_widget<Wid: WidgetExt>(wid: &Wid, event_x: i32, event_y: i32) -> bool {
    let x = wid.x();
    let y = wid.y(); 
    let w = wid.width();
    let h = wid.height();
    if app::event_inside(x,y,w,h){
        return true
    } else {
        return false
    }
}

// pub fn debug_widg<Wid: WidgetExt>(wid: &Wid, col: Color){
//     let wc = wid.clone();
//     wc.set_frame(FrameType::BorderFrame);
//     wc.set_color(col);
// }