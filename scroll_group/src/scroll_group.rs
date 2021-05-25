use fltk::{app, button::*, frame::*, group::*, draw::*, valuator::*};
use std::ops::{Deref, DerefMut};
// use std::cmp::*;
// use syn::*;
use super::Message;

#[derive(Clone)]
pub struct ScrollGroup {
    pub pack: Group, 
    pub hscrollbar: HScrollBar,
}
#[derive(Clone)]
pub struct HScrollBar{
    pub frame: Frame
}

impl HScrollBar{
    pub fn new(x: i32, y: i32)->Self{
        // let mut container = Pack::new(0,0,0,0);
        let mut frame = Frame::new(x,y,20,20,"");
        frame.set_frame(FrameType::RFlatBox);
        frame.set_color(Color::White);

        let mut hb = HScrollBar{
            frame: frame
        };

        hb
    }
}
impl ScrollGroup {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut container = Group::new(x,y,w,h,"");
        container.end();
        container.set_frame(FrameType::BorderFrame);
        container.set_color(Color::Red);

        // container.set_frame(FrameType::BorderFrame);
        let mut bg = Frame::new(x,y-100,w,h,"");
        bg.set_frame(FrameType::FlatBox);
        bg.set_color(Color::Red);
        container.add(&bg);
        container.set_clip_children(true);
        // let mut test_frame÷
        // buttons_cont
        let mut buttons_container = Pack::new(x,y,7000,h,"");
        // buttonS_container.set_clip_children(true);
        buttons_container.set_type(PackType::Horizontal);
        buttons_container.set_frame(FrameType::BorderFrame);
        buttons_container.set_color(Color::Blue);
        // butt
        // buttons_c
        buttons_container.end();

        let hbar = HScrollBar::new(0, y+h-25);
        
        for x in 0..10{
            buttons_container.add(&Button::default().with_size(7000/10,w/5).with_label(&x.to_string()));
        }
        // buttons_container.draw2(move|widg|{
        //     push_clip(x,y,w,h);
        //     for x in 0..widg.children(){
        //         fltk::group::GroupExt::child<fltk::widget::Widget>(x);
        //         // let base = widg.child(x).into_widget();
        //         // let base = widg.child(x).unwrap().into_widget();
        //         widg.draw_child(&mut widg.child(x).unwrap());
        //     }
        //     pop_clip();
        // });
        container.draw2(move|widg|{
            set_clip_region(x,y,w,h);
            push_clip(x,y,w,h);
            for x in 0..widg.children(){
     
                // fltk::group::GroupExt::child<fltk::widget::Widget>(x);
                widg.draw_child(&mut widg.child(x).unwrap());
            }
            pop_clip();
        });
        container.add(&buttons_container);
        container.add(&hbar.frame);
        let mut sg = ScrollGroup {
            pack: container,
            hscrollbar: hbar,
        };

        
        let mut pack_c = buttons_container.clone();
        let bar_c = sg.hscrollbar.frame.clone();
        let cont_cl = container.clone();
        sg.pack.handle2(move |t, ev| {
            match ev {
                Event::MouseWheel =>{
                    let mut children_width = 0;
                    let pack_x = pack_c.x();
                    let pack_y = pack_c.y();
                    let pack_w = pack_c.width();
                    let dy = app::event_dy();
                    
                    dbg!(pack_c.width());
                    if app::event_dy() < 0 && pack_x > -1*pack_c.width()+w{
                        sg.set_damage_type(Damage::Scroll);
                        cont_cl.set_damage_type(Damage::Scroll);
                        pack_c.resize(std::cmp::max(-1*pack_w+w, pack_x+dy*10), pack_y, pack_w, pack_c.height());
                        pack_c.redraw();
                    }
                    if app::event_dy() > 0 && pack_x < 0{
                        sg.set_damage_type(Damage::Scroll);
                        cont_cl.set_damage_type(Damage::Scroll);
                            pack_c.resize(std::cmp::min(0, pack_x+dy*10), pack_y, pack_w, pack_c.height());
                        pack_c.redraw();
                    }
          
                    true
                }
                _=>false 
            }
        });
            
        sg
    }
}

impl Deref for ScrollGroup {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for ScrollGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}


impl Deref for HScrollBar {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}

impl DerefMut for HScrollBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame
    }
}