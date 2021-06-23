use fltk::frame::*;
use fltk::{app::*, image::*, group::*, input::*, draw::*};
use std::ops::{Deref, DerefMut};

// use syn::*;
use super::Message;

#[derive(Clone)]
pub struct SubComponent {
    pub pack: Pack, 
    pub frame: Frame,
}

impl SubComponent {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let icon_w = 24; 
        let input_w = w-icon_w;
        let mut sb = SubComponent {
            pack: Pack::new(x,y,w,h,""),
            frame: Frame::new(x,y,w,h,"")
        };
        
        
        //pack styling
        sb.pack.end();
        sb.frame.draw2(move |b|{
            // draw something
        });

        sb.frame.set_callback2(move |b| {
            // do a callback
        });

        
        sb.frame.handle2(move |t, ev| {
            match ev {
                //handle an event
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

impl Deref for SubComponent {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for SubComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}