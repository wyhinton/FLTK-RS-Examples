use fltk::{app::*, image::*, frame::*, group::*, input::*, draw::*};
use std::ops::{Deref, DerefMut};

// use syn::*;
use super::Message;
mod sub_component;
use sub_component::SubComponent; 
#[derive(Clone)]
pub struct Component {
    pub pack: Pack, 
    pub frame: Frame,
}

impl Component {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let icon_w = 24; 
        let input_w = w-icon_w;
        let mut sb = Component {
            pack: Pack::new(x,y,w,h,""),
            frame: Frame::new(x,y,w,h,"")
        };
        
        
        //pack styling
        sb.pack.end();
        sb.frame.draw2(move |b|{
            set_draw_color(Color::Black);
            //draw something
        });

        sb.frame.set_callback2(move |b| {
            //do a callback
        });

        sb.frame.handle2(move |t, ev| {
            match ev {
                Event::KeyDown =>{
                    println!("{}", "got a key down" );
                    t.do_callback();
                    true
                }
                _=>false 
            }
        });
            
        sb
    }
}

impl Deref for Component {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for Component {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}